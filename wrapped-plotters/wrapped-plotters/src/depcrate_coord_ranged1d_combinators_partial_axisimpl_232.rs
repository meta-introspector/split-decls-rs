// Generated macro for impl_232 (impl)
macro_rules! Depcrate_coord_ranged1d_combinators_partial_axisimpl_232 {
() => {
// Module: crate::coord::ranged1d::combinators::partial_axis
// Provides: {"impl_232"}
// Dependencies: {}
impl < R : DiscreteRanged > DiscreteRanged for PartialAxis < R > where R : Ranged , < R as Ranged > :: ValueType : Eq + Clone , { fn size (& self) -> usize { self . 0 . size () } fn index_of (& self , value : & R :: ValueType) -> Option < usize > { self . 0 . index_of (value) } fn from_index (& self , index : usize) -> Option < Self :: ValueType > { self . 0 . from_index (index) } }
};
}
