// Generated macro for impl_220 (impl)
macro_rules! Depcrate_coord_ranged1d_combinators_nestedimpl_220 {
() => {
// Module: crate::coord::ranged1d::combinators::nested
// Provides: {"impl_220"}
// Dependencies: {}
impl < P : DiscreteRanged , S : DiscreteRanged > DiscreteRanged for NestedRange < P , S > { fn size (& self) -> usize { self . secondary . iter () . map (| x | x . size ()) . sum :: < usize > () } fn index_of (& self , value : & Self :: ValueType) -> Option < usize > { let p_idx = self . primary . index_of (value . category ()) ? ; let s_idx = self . secondary [p_idx] . index_of (value . nested_value () ?) ? ; Some (s_idx + self . secondary [.. p_idx] . iter () . map (| x | x . size ()) . sum :: < usize > () ,) } fn from_index (& self , mut index : usize) -> Option < Self :: ValueType > { for (p_idx , snd) in self . secondary . iter () . enumerate () { if snd . size () > index { return Some (NestedValue :: Value (self . primary . from_index (p_idx) . unwrap () , snd . from_index (index) . unwrap () ,)) ; } index -= snd . size () ; } None } }
};
}
