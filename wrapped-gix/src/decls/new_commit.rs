macro_rules! deps {
    () => {
        Committer!();
        Author!();
        Error!();
    };
}

macro_rules! new_commit {
    () => {
        deps!();
        # [doc = ""] mod new_commit { # [doc = " The error returned by [`new_commit(…)`](crate::Repository::new_commit())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] ParseTime (# [from] crate :: config :: time :: Error) , # [error ("Committer identity is not configured")] CommitterMissing , # [error ("Author identity is not configured")] AuthorMissing , # [error (transparent)] NewCommitAs (# [from] crate :: repository :: new_commit_as :: Error) , } }
    };
}

new_commit!();