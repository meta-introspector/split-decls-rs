// Generated macro for impl_341 (impl)
macro_rules! Depcrate_coord_ranged1d_types_sliceimpl_341 {
() => {
// Module: crate::coord::ranged1d::types::slice
// Provides: {"impl_341"}
// Dependencies: {}
impl < 'a , T : PartialEq > AsRangedCoord for & 'a [T] { type CoordDescType = RangedSlice < 'a , T > ; type Value = & 'a T ; }
};
}
