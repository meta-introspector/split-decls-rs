// Generated macro for init (module)
macro_rules! Depcrate_pathspecinit {
() => {
// Module: crate::pathspec
// Provides: {"init"}
// Dependencies: {}
# [doc = ""] pub mod init { # [doc = " The error returned by [`Pathspec::new()`](super::Pathspec::new())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] MakeAttributes (# [from] Box < dyn std :: error :: Error + Send + Sync + 'static >) , # [error (transparent)] Defaults (# [from] crate :: repository :: pathspec_defaults_ignore_case :: Error) , # [error (transparent)] ParseSpec (# [from] gix_pathspec :: parse :: Error) , # [error ("Could not obtain the repository prefix as the relative path of the CWD as seen from the working tree")] NormalizeSpec (# [from] gix_pathspec :: normalize :: Error) , # [error (transparent)] RepoPrefix (# [from] gix_path :: realpath :: Error) , } }
};
}
