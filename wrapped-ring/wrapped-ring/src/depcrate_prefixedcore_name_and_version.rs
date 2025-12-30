// Generated macro for core_name_and_version (macro)
macro_rules! Depcrate_prefixedcore_name_and_version {
() => {
// Module: crate::prefixed
// Provides: {"core_name_and_version"}
// Dependencies: {}
macro_rules ! core_name_and_version { () => { concat ! (env ! ("CARGO_PKG_NAME") , "_core_" , env ! ("CARGO_PKG_VERSION_MAJOR") , "_" , env ! ("CARGO_PKG_VERSION_MINOR") , "_" , env ! ("CARGO_PKG_VERSION_PATCH") , "_" , env ! ("CARGO_PKG_VERSION_PRE") ,) } ; }
};
}
