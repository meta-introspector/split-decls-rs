// Generated macro for impl_566 (impl)
macro_rules! Depcrate_types_markerimpl_566 {
() => {
// Module: crate::types::marker
// Provides: {"impl_566"}
// Dependencies: {}
impl < S , T > IsInputType < S > for Option < T > where T : IsInputType < S > , S : ScalarValue , { # [inline] fn mark () { T :: mark () } }
};
}
