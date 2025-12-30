// Generated macro for impl_1887 (impl)
macro_rules! Depcrate_vec_spec_from_elemimpl_1887 {
() => {
// Module: crate::vec::spec_from_elem
// Provides: {"impl_1887"}
// Dependencies: {}
impl SpecFromElem for () { # [inline] fn from_elem < A : Allocator > (_elem : () , n : usize , alloc : A) -> Vec < () , A > { let mut v = Vec :: with_capacity_in (n , alloc) ; unsafe { v . set_len (n) ; } v } }
};
}
