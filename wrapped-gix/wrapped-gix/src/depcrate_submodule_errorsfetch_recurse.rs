// Generated macro for fetch_recurse (module)
macro_rules! Depcrate_submodule_errorsfetch_recurse {
() => {
// Module: crate::submodule::errors
// Provides: {"fetch_recurse"}
// Dependencies: {}
# [doc = ""] pub mod fetch_recurse { # [doc = " The error returned by [Submodule::fetch_recurse()](crate::Submodule::fetch_recurse())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] ModuleBoolean (# [from] gix_submodule :: config :: Error) , # [error (transparent)] ConfigurationFallback (# [from] crate :: config :: key :: GenericErrorWithValue) , } }
};
}
