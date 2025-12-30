// Generated macro for excludes (module)
macro_rules! Depcrate_worktreeexcludes {
() => {
// Module: crate::worktree
// Provides: {"excludes"}
// Dependencies: {}
# [doc = ""] # [cfg (feature = "excludes")] pub mod excludes { use crate :: AttributeStack ; # [doc = " The error returned by [`Worktree::excludes()`][crate::Worktree::excludes()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] OpenIndex (# [from] crate :: worktree :: open_index :: Error) , # [error (transparent)] CreateCache (# [from] crate :: config :: exclude_stack :: Error) , } impl crate :: Worktree < '_ > { # [doc = " Configure a file-system cache checking if files below the repository are excluded."] # [doc = ""] # [doc = " This takes into consideration all the usual repository configuration, namely:"] # [doc = ""] # [doc = " * `$XDG_CONFIG_HOME/…/ignore` if `core.excludesFile` is *not* set, otherwise use the configured file."] # [doc = " * `$GIT_DIR/info/exclude` if present."] # [doc = ""] # [doc = " When only excludes are desired, this is the most efficient way to obtain them. Otherwise use"] # [doc = " [`Worktree::attributes()`][crate::Worktree::attributes()] for accessing both attributes and excludes."] pub fn excludes (& self , overrides : Option < gix_ignore :: Search >) -> Result < AttributeStack < '_ > , Error > { let index = self . index () ? ; Ok (self . parent . excludes (& index , overrides , gix_worktree :: stack :: state :: ignore :: Source :: WorktreeThenIdMappingIfNotSkipped ,) ?) } } }
};
}
