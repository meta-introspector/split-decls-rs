// Generated macro for impl_567 (impl)
macro_rules! Depcrate_types_markerimpl_567 {
() => {
// Module: crate::types::marker
// Provides: {"impl_567"}
// Dependencies: {}
impl < S , T > IsInputType < S > for Vec < T > where T : IsInputType < S > , S : ScalarValue , { # [inline] fn mark () { T :: mark () } }
};
}
