// Generated macro for impl_198 (impl)
macro_rules! Depcrate_astimpl_198 {
() => {
// Module: crate::ast
// Provides: {"impl_198"}
// Dependencies: {}
impl < N : AstNode > fmt :: Debug for AstPtr < N > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("AstPtr") . field ("raw" , & self . raw) . finish () } }
};
}
