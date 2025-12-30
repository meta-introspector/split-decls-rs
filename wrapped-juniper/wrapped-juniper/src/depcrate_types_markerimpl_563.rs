// Generated macro for impl_563 (impl)
macro_rules! Depcrate_types_markerimpl_563 {
() => {
// Module: crate::types::marker
// Provides: {"impl_563"}
// Dependencies: {}
impl < S , T > IsInputType < S > for & T where T : IsInputType < S > + ? Sized , S : ScalarValue , { # [inline] fn mark () { T :: mark () } }
};
}
