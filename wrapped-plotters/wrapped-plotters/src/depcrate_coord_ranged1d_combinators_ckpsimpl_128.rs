// Generated macro for impl_128 (impl)
macro_rules! Depcrate_coord_ranged1d_combinators_ckpsimpl_128 {
() => {
// Module: crate::coord::ranged1d::combinators::ckps
// Provides: {"impl_128"}
// Dependencies: {}
impl < R : DiscreteRanged > DiscreteRanged for WithKeyPoints < R > where R :: ValueType : Clone , { fn size (& self) -> usize { self . inner . size () } fn index_of (& self , value : & Self :: ValueType) -> Option < usize > { self . inner . index_of (value) } fn from_index (& self , index : usize) -> Option < Self :: ValueType > { self . inner . from_index (index) } }
};
}
