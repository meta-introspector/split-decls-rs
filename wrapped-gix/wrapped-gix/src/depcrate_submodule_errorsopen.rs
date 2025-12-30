// Generated macro for open (module)
macro_rules! Depcrate_submodule_errorsopen {
() => {
// Module: crate::submodule::errors
// Provides: {"open"}
// Dependencies: {}
# [doc = ""] pub mod open { # [doc = " The error returned by [Submodule::open()](crate::Submodule::open())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] OpenRepository (# [from] crate :: open :: Error) , # [error (transparent)] PathConfiguration (# [from] gix_submodule :: config :: path :: Error) , } }
};
}
