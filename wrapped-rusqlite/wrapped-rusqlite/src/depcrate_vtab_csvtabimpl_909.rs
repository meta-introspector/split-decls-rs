// Generated macro for impl_909 (impl)
macro_rules! Depcrate_vtab_csvtabimpl_909 {
() => {
// Module: crate::vtab::csvtab
// Provides: {"impl_909"}
// Dependencies: {}
impl From < csv :: Error > for Error { # [cold] fn from (err : csv :: Error) -> Self { Self :: ModuleError (err . to_string ()) } }
};
}
