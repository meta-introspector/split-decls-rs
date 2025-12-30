// Generated macro for impl_1885 (impl)
macro_rules! Depcrate_vec_spec_from_elemimpl_1885 {
() => {
// Module: crate::vec::spec_from_elem
// Provides: {"impl_1885"}
// Dependencies: {}
impl SpecFromElem for i8 { # [inline] # [track_caller] fn from_elem < A : Allocator > (elem : i8 , n : usize , alloc : A) -> Vec < i8 , A > { if elem == 0 { return Vec { buf : RawVec :: with_capacity_zeroed_in (n , alloc) , len : n } ; } let mut v = Vec :: with_capacity_in (n , alloc) ; unsafe { ptr :: write_bytes (v . as_mut_ptr () , elem as u8 , n) ; v . set_len (n) ; } v } }
};
}
