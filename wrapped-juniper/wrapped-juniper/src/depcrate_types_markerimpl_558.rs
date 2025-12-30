// Generated macro for impl_558 (impl)
macro_rules! Depcrate_types_markerimpl_558 {
() => {
// Module: crate::types::marker
// Provides: {"impl_558"}
// Dependencies: {}
impl < S , T > IsOutputType < S > for Vec < T > where T : IsOutputType < S > , S : ScalarValue , { # [inline] fn mark () { T :: mark () } }
};
}
