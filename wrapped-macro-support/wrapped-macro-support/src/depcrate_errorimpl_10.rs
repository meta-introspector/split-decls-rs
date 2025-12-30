// Generated macro for impl_10 (impl)
macro_rules! Depcrate_errorimpl_10 {
() => {
// Module: crate::error
// Provides: {"impl_10"}
// Dependencies: {}
impl From < Error > for Diagnostic { fn from (err : Error) -> Diagnostic { Diagnostic { inner : Repr :: SynError (err) , } } }
};
}
