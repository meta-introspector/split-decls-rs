// Generated macro for impl_30 (impl)
macro_rules! Depcrate_error_std_implsimpl_30 {
() => {
// Module: crate::error_std_impls
// Provides: {"impl_30"}
// Dependencies: {}
impl From < Error > for io :: Error { fn from (err : Error) -> Self { match err . raw_os_error () { Some (errno) => io :: Error :: from_raw_os_error (errno) , None => io :: Error :: other (err) , } } }
};
}
