macro_rules! deps {
    () => {
        Configuration!();
        Driver!();
        Error!();
    };
}

macro_rules! to_worktree {
    () => {
        deps!();
        # [doc = ""] pub mod to_worktree { # [doc = " The error returned by [Pipeline::convert_to_worktree()][super::Pipeline::convert_to_worktree()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Ident (# [from] crate :: ident :: apply :: Error) , # [error (transparent)] Eol (# [from] crate :: eol :: convert_to_worktree :: Error) , # [error (transparent)] Worktree (# [from] crate :: worktree :: encode_to_worktree :: Error) , # [error (transparent)] Driver (# [from] crate :: driver :: apply :: Error) , # [error (transparent)] Configuration (# [from] super :: configuration :: Error) , } }
    };
}

to_worktree!()