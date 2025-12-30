// Generated macro for impl_1931 (impl)
macro_rules! Depcrate_vec_spec_extendimpl_1931 {
() => {
// Module: crate::vec::spec_extend
// Provides: {"impl_1931"}
// Dependencies: {}
impl < T , A : Allocator > SpecExtend < T , IntoIter < T > > for Vec < T , A > { # [track_caller] fn spec_extend (& mut self , mut iterator : IntoIter < T >) { unsafe { self . append_elements (iterator . as_slice () as _) ; } iterator . forget_remaining_elements () ; } }
};
}
