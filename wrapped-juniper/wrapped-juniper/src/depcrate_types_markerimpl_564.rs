// Generated macro for impl_564 (impl)
macro_rules! Depcrate_types_markerimpl_564 {
() => {
// Module: crate::types::marker
// Provides: {"impl_564"}
// Dependencies: {}
impl < S , T > IsInputType < S > for Box < T > where T : IsInputType < S > + ? Sized , S : ScalarValue , { # [inline] fn mark () { T :: mark () } }
};
}
