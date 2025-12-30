// Generated macro for impl_295 (impl)
macro_rules! Depcrate_ast_traitsimpl_295 {
() => {
// Module: crate::ast_traits
// Provides: {"impl_295"}
// Dependencies: {}
impl < Wrapped : fmt :: Debug , Tag > fmt :: Debug for AstNodeWrapper < Wrapped , Tag > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("AstNodeWrapper") . field ("wrapped" , & self . wrapped) . field ("tag" , & self . tag) . finish () } }
};
}
