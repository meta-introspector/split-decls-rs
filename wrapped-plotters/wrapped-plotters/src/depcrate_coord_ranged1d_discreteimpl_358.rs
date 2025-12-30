// Generated macro for impl_358 (impl)
macro_rules! Depcrate_coord_ranged1d_discreteimpl_358 {
() => {
// Module: crate::coord::ranged1d::discrete
// Provides: {"impl_358"}
// Dependencies: {}
impl < 'a , T : DiscreteRanged > Iterator for DiscreteValueIter < 'a , T > { type Item = T :: ValueType ; fn next (& mut self) -> Option < T :: ValueType > { if self . 1 >= self . 2 { return None ; } let idx = self . 1 ; self . 1 += 1 ; self . 0 . from_index (idx) } }
};
}
