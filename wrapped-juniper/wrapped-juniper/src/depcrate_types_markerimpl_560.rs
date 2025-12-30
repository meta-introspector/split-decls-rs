// Generated macro for impl_560 (impl)
macro_rules! Depcrate_types_markerimpl_560 {
() => {
// Module: crate::types::marker
// Provides: {"impl_560"}
// Dependencies: {}
impl < S , T , const N : usize > IsOutputType < S > for [T ; N] where T : IsOutputType < S > , S : ScalarValue , { # [inline] fn mark () { T :: mark () } }
};
}
