// Generated macro for impl_555 (impl)
macro_rules! Depcrate_types_markerimpl_555 {
() => {
// Module: crate::types::marker
// Provides: {"impl_555"}
// Dependencies: {}
impl < S , T > IsOutputType < S > for Box < T > where T : IsOutputType < S > + ? Sized , S : ScalarValue , { # [inline] fn mark () { T :: mark () } }
};
}
