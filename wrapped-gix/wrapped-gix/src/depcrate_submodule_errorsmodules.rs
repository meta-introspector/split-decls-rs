// Generated macro for modules (module)
macro_rules! Depcrate_submodule_errorsmodules {
() => {
// Module: crate::submodule::errors
// Provides: {"modules"}
// Dependencies: {}
# [doc = ""] pub mod modules { # [doc = " The error returned by [Repository::modules()](crate::Repository::modules())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] OpenModulesFile (# [from] crate :: submodule :: open_modules_file :: Error) , # [error (transparent)] OpenIndex (# [from] crate :: worktree :: open_index :: Error) , # [error ("Could not find the .gitmodules file by id in the object database")] FindExistingBlob (# [from] crate :: object :: find :: existing :: Error) , # [error (transparent)] FindHeadRef (# [from] crate :: reference :: find :: existing :: Error) , # [error (transparent)] PeelHeadRef (# [from] crate :: head :: peel :: Error) , # [error (transparent)] PeelObjectToCommit (# [from] crate :: object :: peel :: to_kind :: Error) , # [error (transparent)] TreeFromCommit (# [from] crate :: object :: commit :: Error) , } }
};
}
