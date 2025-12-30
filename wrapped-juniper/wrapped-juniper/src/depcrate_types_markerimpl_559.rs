// Generated macro for impl_559 (impl)
macro_rules! Depcrate_types_markerimpl_559 {
() => {
// Module: crate::types::marker
// Provides: {"impl_559"}
// Dependencies: {}
impl < S , T > IsOutputType < S > for [T] where T : IsOutputType < S > , S : ScalarValue , { # [inline] fn mark () { T :: mark () } }
};
}
