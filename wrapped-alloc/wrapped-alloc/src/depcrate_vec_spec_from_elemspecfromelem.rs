// Generated macro for SpecFromElem (trait)
macro_rules! Depcrate_vec_spec_from_elemSpecFromElem {
() => {
// Module: crate::vec::spec_from_elem
// Provides: {"SpecFromElem"}
// Dependencies: {}
pub (super) trait SpecFromElem : Sized { fn from_elem < A : Allocator > (elem : Self , n : usize , alloc : A) -> Vec < Self , A > ; }
};
}
