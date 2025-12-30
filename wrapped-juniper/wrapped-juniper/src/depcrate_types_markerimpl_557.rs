// Generated macro for impl_557 (impl)
macro_rules! Depcrate_types_markerimpl_557 {
() => {
// Module: crate::types::marker
// Provides: {"impl_557"}
// Dependencies: {}
impl < S , T > IsOutputType < S > for Option < T > where T : IsOutputType < S > , S : ScalarValue , { # [inline] fn mark () { T :: mark () } }
};
}
