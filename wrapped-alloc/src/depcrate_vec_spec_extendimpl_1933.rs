// Generated macro for impl_1933 (impl)
macro_rules! Depcrate_vec_spec_extendimpl_1933 {
() => {
// Module: crate::vec::spec_extend
// Provides: {"impl_1933"}
// Dependencies: {}
impl < 'a , T : 'a , A : Allocator > SpecExtend < & 'a T , slice :: Iter < 'a , T > > for Vec < T , A > where T : Copy , { # [track_caller] fn spec_extend (& mut self , iterator : slice :: Iter < 'a , T >) { let slice = iterator . as_slice () ; unsafe { self . append_elements (slice) } ; } }
};
}
