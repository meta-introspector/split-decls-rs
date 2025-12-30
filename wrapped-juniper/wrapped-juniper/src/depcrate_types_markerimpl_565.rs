// Generated macro for impl_565 (impl)
macro_rules! Depcrate_types_markerimpl_565 {
() => {
// Module: crate::types::marker
// Provides: {"impl_565"}
// Dependencies: {}
impl < S , T > IsInputType < S > for Arc < T > where T : IsInputType < S > + ? Sized , S : ScalarValue , { # [inline] fn mark () { T :: mark () } }
};
}
