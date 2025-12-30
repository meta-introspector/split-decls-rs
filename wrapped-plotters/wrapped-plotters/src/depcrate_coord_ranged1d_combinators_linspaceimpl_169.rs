// Generated macro for impl_169 (impl)
macro_rules! Depcrate_coord_ranged1d_combinators_linspaceimpl_169 {
() => {
// Module: crate::coord::ranged1d::combinators::linspace
// Provides: {"impl_169"}
// Dependencies: {}
impl < T : Ranged , S : Clone , R : LinspaceRoundingMethod < T :: ValueType > > DiscreteRanged for Linspace < T , S , R > where T :: ValueType : Add < S , Output = T :: ValueType > + PartialOrd + Clone , { fn size (& self) -> usize { self . grid_value . len () } fn index_of (& self , value : & T :: ValueType) -> Option < usize > { R :: search (self . grid_value . as_ref () , value) } fn from_index (& self , idx : usize) -> Option < T :: ValueType > { self . grid_value . get (idx) . cloned () } }
};
}
