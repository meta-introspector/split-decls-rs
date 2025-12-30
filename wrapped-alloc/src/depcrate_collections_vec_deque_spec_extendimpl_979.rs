// Generated macro for impl_979 (impl)
macro_rules! Depcrate_collections_vec_deque_spec_extendimpl_979 {
() => {
// Module: crate::collections::vec_deque::spec_extend
// Provides: {"impl_979"}
// Dependencies: {}
impl < 'a , T : 'a , I , A : Allocator > SpecExtend < & 'a T , I > for VecDeque < T , A > where I : Iterator < Item = & 'a T > , T : Copy , { # [track_caller] default fn spec_extend (& mut self , iterator : I) { self . spec_extend (iterator . copied ()) } }
};
}
