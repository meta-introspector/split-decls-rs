// Generated macro for impl_554 (impl)
macro_rules! Depcrate_types_markerimpl_554 {
() => {
// Module: crate::types::marker
// Provides: {"impl_554"}
// Dependencies: {}
impl < S , T > IsOutputType < S > for & T where T : IsOutputType < S > + ? Sized , S : ScalarValue , { # [inline] fn mark () { T :: mark () } }
};
}
