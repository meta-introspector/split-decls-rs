// Generated macro for impl_556 (impl)
macro_rules! Depcrate_types_markerimpl_556 {
() => {
// Module: crate::types::marker
// Provides: {"impl_556"}
// Dependencies: {}
impl < S , T > IsOutputType < S > for Arc < T > where T : IsOutputType < S > + ? Sized , S : ScalarValue , { # [inline] fn mark () { T :: mark () } }
};
}
