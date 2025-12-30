// Generated macro for PathspecDetached (struct)
macro_rules! Depcrate_typesPathspecDetached {
() => {
// Module: crate::types
// Provides: {"PathspecDetached"}
// Dependencies: {}
# [doc = " Like [`Pathspec`], but without a Repository reference and with minimal API."] # [derive (Clone)] # [cfg (feature = "attributes")] pub struct PathspecDetached { # [doc = " The cache to power attribute access. It's only initialized if we have a pattern with attributes."] pub stack : Option < gix_worktree :: Stack > , # [doc = " The prepared search to use for checking matches."] pub search : gix_pathspec :: Search , # [doc = " A thread-safe version of an ODB."] pub odb : crate :: OdbHandleArc , }
};
}
