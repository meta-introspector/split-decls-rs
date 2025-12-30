// Generated macro for Error (enum)
macro_rules! Depcrate_fetch_refmap_initError {
() => {
// Module: crate::fetch::refmap::init
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [`RefMap::fetch()`]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("The object format {format:?} as used by the remote is unsupported")] UnknownObjectFormat { format : BString } , # [error (transparent)] MappingValidation (# [from] gix_refspec :: match_group :: validate :: Error) , # [error (transparent)] ListRefs (# [from] crate :: ls_refs :: Error) , }
};
}
