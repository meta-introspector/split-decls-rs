// Generated macro for impl_113 (impl)
macro_rules! Depcrate_canonicalimpl_113 {
() => {
// Module: crate::canonical
// Provides: {"impl_113"}
// Dependencies: {}
impl < A > From < NodeStmt < A > > for Node < A > { fn from (stmt : NodeStmt < A >) -> Self { Node { id : stmt . node . id . to_string () , port : stmt . node . port , attr : stmt . attr . map (| list | list . into ()) . unwrap_or (AList :: empty ()) , } } }
};
}
