// Generated macro for cp (function)
macro_rules! Depcrate_fscp {
() => {
// Module: crate::fs
// Provides: {"cp"}
// Dependencies: {}
pub fn cp (from : & Path , to : & Path) -> Result < () > { fs :: copy (from , to) . map_err (| inner | Error :: CopyError { inner , from : from . to_owned () , to : to . to_owned () , }) ? ; Ok (()) }
};
}
