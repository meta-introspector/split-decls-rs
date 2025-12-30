// Generated macro for impl_1383 (impl)
macro_rules! Depcrate_default_numeric_fallbackimpl_1383 {
() => {
// Module: crate::default_numeric_fallback
// Provides: {"impl_1383"}
// Dependencies: {}
impl < 'tcx > From < Option < Ty < 'tcx > > > for ExplicitTyBound { fn from (v : Option < Ty < 'tcx > >) -> Self { Self (v . is_some_and (Ty :: is_numeric)) } }
};
}
