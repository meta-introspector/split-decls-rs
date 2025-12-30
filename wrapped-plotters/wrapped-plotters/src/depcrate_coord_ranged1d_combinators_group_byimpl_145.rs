// Generated macro for impl_145 (impl)
macro_rules! Depcrate_coord_ranged1d_combinators_group_byimpl_145 {
() => {
// Module: crate::coord::ranged1d::combinators::group_by
// Provides: {"impl_145"}
// Dependencies: {}
impl < T : DiscreteRanged > DiscreteRanged for GroupBy < T > { fn size (& self) -> usize { (self . 0 . size () + self . 1 - 1) / self . 1 } fn index_of (& self , value : & Self :: ValueType) -> Option < usize > { self . 0 . index_of (value) . map (| idx | idx / self . 1) } fn from_index (& self , index : usize) -> Option < Self :: ValueType > { self . 0 . from_index (index * self . 1) } }
};
}
