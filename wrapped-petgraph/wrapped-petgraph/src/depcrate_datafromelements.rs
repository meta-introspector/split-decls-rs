// Generated macro for FromElements (trait)
macro_rules! Depcrate_dataFromElements {
() => {
// Module: crate::data
// Provides: {"FromElements"}
// Dependencies: {}
# [doc = " Create a graph from an iterator of elements."] pub trait FromElements : Create { fn from_elements < I > (iterable : I) -> Self where Self : Sized , I : IntoIterator < Item = Element < Self :: NodeWeight , Self :: EdgeWeight > > , { let mut gr = Self :: with_capacity (0 , 0) ; let mut map = Vec :: new () ; for element in iterable { match element { Element :: Node { weight } => { map . push (gr . add_node (weight)) ; } Element :: Edge { source , target , weight , } => { gr . add_edge (map [source] , map [target] , weight) ; } } } gr } }
};
}
