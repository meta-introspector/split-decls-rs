// Generated macro for impl_36 (impl)
macro_rules! Depcrate_errorimpl_36 {
() => {
// Module: crate::error
// Provides: {"impl_36"}
// Dependencies: {}
impl From < Error > for io :: Error { fn from (err : Error) -> Self { io :: Error :: other (err . to_string ()) } }
};
}
