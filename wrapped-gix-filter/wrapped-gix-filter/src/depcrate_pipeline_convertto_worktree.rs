// Generated macro for to_worktree (module)
macro_rules! Depcrate_pipeline_convertto_worktree {
() => {
// Module: crate::pipeline::convert
// Provides: {"to_worktree"}
// Dependencies: {}
# [doc = ""] pub mod to_worktree { # [doc = " The error returned by [Pipeline::convert_to_worktree()][super::Pipeline::convert_to_worktree()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Ident (# [from] crate :: ident :: apply :: Error) , # [error (transparent)] Eol (# [from] crate :: eol :: convert_to_worktree :: Error) , # [error (transparent)] Worktree (# [from] crate :: worktree :: encode_to_worktree :: Error) , # [error (transparent)] Driver (# [from] crate :: driver :: apply :: Error) , # [error (transparent)] Configuration (# [from] super :: configuration :: Error) , } }
};
}
