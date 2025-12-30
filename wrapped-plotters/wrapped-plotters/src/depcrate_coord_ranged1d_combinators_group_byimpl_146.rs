// Generated macro for impl_146 (impl)
macro_rules! Depcrate_coord_ranged1d_combinators_group_byimpl_146 {
() => {
// Module: crate::coord::ranged1d::combinators::group_by
// Provides: {"impl_146"}
// Dependencies: {}
impl < T , R : DiscreteRanged < ValueType = T > + ValueFormatter < T > > ValueFormatter < T > for GroupBy < R > { fn format (value : & T) -> String { R :: format (value) } }
};
}
