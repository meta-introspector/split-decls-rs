// Generated macro for impl_60 (impl)
macro_rules! Depcrate_iterators_flat_pairsimpl_60 {
() => {
// Module: crate::iterators::flat_pairs
// Provides: {"impl_60"}
// Dependencies: {}
impl < R : RuleType > ExactSizeIterator for FlatPairs < '_ , R > { fn len (& self) -> usize { (self . end - self . start) >> 1 } }
};
}
