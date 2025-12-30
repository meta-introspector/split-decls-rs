// Generated macro for impl_7587 (impl)
macro_rules! Depcrate_mutable_debug_assertionimpl_7587 {
() => {
// Module: crate::mutable_debug_assertion
// Provides: {"impl_7587"}
// Dependencies: {}
impl < 'a , 'tcx > MutArgVisitor < 'a , 'tcx > { fn new (cx : & 'a LateContext < 'tcx >) -> Self { Self { cx , expr_span : None , found : false , } } fn expr_span (& self) -> Option < Span > { if self . found { self . expr_span } else { None } } }
};
}
