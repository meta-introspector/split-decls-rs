// Generated macro for impl_569 (impl)
macro_rules! Depcrate_types_markerimpl_569 {
() => {
// Module: crate::types::marker
// Provides: {"impl_569"}
// Dependencies: {}
impl < S , T , const N : usize > IsInputType < S > for [T ; N] where T : IsInputType < S > , S : ScalarValue , { # [inline] fn mark () { T :: mark () } }
};
}
