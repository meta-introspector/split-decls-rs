// Generated macro for impl_1883 (impl)
macro_rules! Depcrate_vec_spec_from_elemimpl_1883 {
() => {
// Module: crate::vec::spec_from_elem
// Provides: {"impl_1883"}
// Dependencies: {}
impl < T : Clone > SpecFromElem for T { # [track_caller] default fn from_elem < A : Allocator > (elem : Self , n : usize , alloc : A) -> Vec < Self , A > { let mut v = Vec :: with_capacity_in (n , alloc) ; v . extend_with (n , elem) ; v } }
};
}
