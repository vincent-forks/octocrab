use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Issue {
    pub id: IssueId,
    pub node_id: Option<String>,
    #[serde(skip_serializing)]
    pub url: Url,
    #[serde(skip_serializing)]
    pub repository_url: Option<Url>,
    #[serde(skip_serializing)]
    pub labels_url: Url,
    #[serde(skip_serializing)]
    pub comments_url: Url,
    #[serde(skip_serializing)]
    pub events_url: Url,
    #[serde(skip_serializing)]
    pub html_url: Option<Url>,
    pub number: u64,
    pub state: IssueState,
    pub state_reason: Option<IssueStateReason>,
    pub title: String,
    #[serde(skip_serializing)]
    pub body: Option<String>,
    #[serde(skip_serializing)]
    pub body_text: Option<String>,
    #[serde(skip_serializing)]
    pub body_html: Option<String>,
    pub user: Author,
    pub labels: Vec<Label>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<Author>,
    pub assignees: Option<Vec<Author>>,
    pub author_association: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestone: Option<Milestone>,
    pub locked: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_lock_reason: Option<String>,
    pub comments: u32,
    #[serde(skip_serializing)]
    pub pull_request: Option<PullRequestLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_by: Option<Author>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Comment {
    pub id: CommentId,
    pub node_id: Option<String>,
    #[serde(skip_serializing)]
    pub url: Url,
    #[serde(skip_serializing)]
    pub html_url: Option<Url>,
    #[serde(skip_serializing)]
    pub issue_url: Option<Url>,
    #[serde(skip_serializing)]
    pub body: Option<String>,
    #[serde(skip_serializing)]
    pub body_text: Option<String>,
    #[serde(skip_serializing)]
    pub body_html: Option<String>,
    pub author_association: Option<AuthorAssociation>,
    pub user: Author,
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum IssueStateReason {
    Completed,
    NotPlanned,
    Reopened,
    Duplicate,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PullRequestLink {
    pub url: Url,
    pub html_url: Option<Url>,
    pub diff_url: Url,
    pub patch_url: Url,
}
