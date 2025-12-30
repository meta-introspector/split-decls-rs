// Generated macro for impl_568 (impl)
macro_rules! Depcrate_types_markerimpl_568 {
() => {
// Module: crate::types::marker
// Provides: {"impl_568"}
// Dependencies: {}
impl < S , T > IsInputType < S > for [T] where T : IsInputType < S > , S : ScalarValue , { # [inline] fn mark () { T :: mark () } }
};
}
