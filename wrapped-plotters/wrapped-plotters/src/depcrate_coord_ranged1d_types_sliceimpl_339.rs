// Generated macro for impl_339 (impl)
macro_rules! Depcrate_coord_ranged1d_types_sliceimpl_339 {
() => {
// Module: crate::coord::ranged1d::types::slice
// Provides: {"impl_339"}
// Dependencies: {}
impl < 'a , T : PartialEq > DiscreteRanged for RangedSlice < 'a , T > { fn size (& self) -> usize { self . 0 . len () } fn index_of (& self , value : & & 'a T) -> Option < usize > { self . 0 . iter () . position (| x | & x == value) } fn from_index (& self , index : usize) -> Option < & 'a T > { if self . 0 . len () <= index { return None ; } Some (& self . 0 [index]) } }
};
}
