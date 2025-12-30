// Generated macro for impl_340 (impl)
macro_rules! Depcrate_coord_ranged1d_types_sliceimpl_340 {
() => {
// Module: crate::coord::ranged1d::types::slice
// Provides: {"impl_340"}
// Dependencies: {}
impl < 'a , T : PartialEq > From < & 'a [T] > for RangedSlice < 'a , T > { fn from (range : & 'a [T]) -> Self { RangedSlice (range) } }
};
}
