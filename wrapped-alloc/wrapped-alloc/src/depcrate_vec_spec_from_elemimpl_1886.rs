// Generated macro for impl_1886 (impl)
macro_rules! Depcrate_vec_spec_from_elemimpl_1886 {
() => {
// Module: crate::vec::spec_from_elem
// Provides: {"impl_1886"}
// Dependencies: {}
impl SpecFromElem for u8 { # [inline] # [track_caller] fn from_elem < A : Allocator > (elem : u8 , n : usize , alloc : A) -> Vec < u8 , A > { if elem == 0 { return Vec { buf : RawVec :: with_capacity_zeroed_in (n , alloc) , len : n } ; } let mut v = Vec :: with_capacity_in (n , alloc) ; unsafe { ptr :: write_bytes (v . as_mut_ptr () , elem , n) ; v . set_len (n) ; } v } }
};
}
