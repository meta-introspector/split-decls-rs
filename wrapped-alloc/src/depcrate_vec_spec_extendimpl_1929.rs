// Generated macro for impl_1929 (impl)
macro_rules! Depcrate_vec_spec_extendimpl_1929 {
() => {
// Module: crate::vec::spec_extend
// Provides: {"impl_1929"}
// Dependencies: {}
impl < T , I , A : Allocator > SpecExtend < T , I > for Vec < T , A > where I : Iterator < Item = T > , { # [track_caller] default fn spec_extend (& mut self , iter : I) { self . extend_desugared (iter) } }
};
}
