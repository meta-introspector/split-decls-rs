// Generated macro for impl_1930 (impl)
macro_rules! Depcrate_vec_spec_extendimpl_1930 {
() => {
// Module: crate::vec::spec_extend
// Provides: {"impl_1930"}
// Dependencies: {}
impl < T , I , A : Allocator > SpecExtend < T , I > for Vec < T , A > where I : TrustedLen < Item = T > , { # [track_caller] default fn spec_extend (& mut self , iterator : I) { self . extend_trusted (iterator) } }
};
}
