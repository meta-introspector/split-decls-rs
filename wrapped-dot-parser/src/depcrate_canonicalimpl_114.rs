// Generated macro for impl_114 (impl)
macro_rules! Depcrate_canonicalimpl_114 {
() => {
// Module: crate::canonical
// Provides: {"impl_114"}
// Dependencies: {}
impl < A > From < & NodeID > for Node < A > { fn from (node : & NodeID) -> Self { Node { id : node . id . to_string () , port : node . port . clone () , attr : AList :: empty () , } } }
};
}
