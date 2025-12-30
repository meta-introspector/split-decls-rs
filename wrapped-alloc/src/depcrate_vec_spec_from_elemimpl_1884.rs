// Generated macro for impl_1884 (impl)
macro_rules! Depcrate_vec_spec_from_elemimpl_1884 {
() => {
// Module: crate::vec::spec_from_elem
// Provides: {"impl_1884"}
// Dependencies: {}
impl < T : Clone + IsZero > SpecFromElem for T { # [inline] # [track_caller] default fn from_elem < A : Allocator > (elem : T , n : usize , alloc : A) -> Vec < T , A > { if elem . is_zero () { return Vec { buf : RawVec :: with_capacity_zeroed_in (n , alloc) , len : n } ; } let mut v = Vec :: with_capacity_in (n , alloc) ; v . extend_with (n , elem) ; v } }
};
}
