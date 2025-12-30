// Generated macro for impl_1932 (impl)
macro_rules! Depcrate_vec_spec_extendimpl_1932 {
() => {
// Module: crate::vec::spec_extend
// Provides: {"impl_1932"}
// Dependencies: {}
impl < 'a , T : 'a , I , A : Allocator > SpecExtend < & 'a T , I > for Vec < T , A > where I : Iterator < Item = & 'a T > , T : Clone , { # [track_caller] default fn spec_extend (& mut self , iterator : I) { self . spec_extend (iterator . cloned ()) } }
};
}
