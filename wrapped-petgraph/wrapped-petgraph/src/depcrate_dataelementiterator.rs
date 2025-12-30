// Generated macro for ElementIterator (trait)
macro_rules! Depcrate_dataElementIterator {
() => {
// Module: crate::data
// Provides: {"ElementIterator"}
// Dependencies: {}
# [doc = " Iterator adaptors for iterators of `Element`."] pub trait ElementIterator < N , E > : Iterator < Item = Element < N , E > > { # [doc = " Create an iterator adaptor that filters graph elements."] # [doc = ""] # [doc = " The function `f` is called with each element and if its return value"] # [doc = " is `true` the element is accepted and if `false` it is removed."] # [doc = " `f` is called with mutable references to the node and edge weights,"] # [doc = " so that they can be mutated (but the edge endpoints can not)."] # [doc = ""] # [doc = " This filter adapts the edge source and target indices in the"] # [doc = " stream so that they are correct after the removals."] fn filter_elements < F > (self , f : F) -> FilterElements < Self , F > where Self : Sized , F : FnMut (Element < & mut N , & mut E >) -> bool , { FilterElements { iter : self , node_index : 0 , map : Vec :: new () , f , } } }
};
}
