// Generated macro for impl_115 (impl)
macro_rules! Depcrate_canonicalimpl_115 {
() => {
// Module: crate::canonical
// Provides: {"impl_115"}
// Dependencies: {}
impl < A > Node < A > { fn map < F , B > (self , f : F) -> Node < B > where F : Fn (A) -> Option < B > , { Node { id : self . id , port : self . port , attr : self . attr . filter_map_attr (& f) , } } }
};
}
