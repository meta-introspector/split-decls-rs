// Generated macro for impl_202 (impl)
macro_rules! Depcrate_astimpl_202 {
() => {
// Module: crate::ast
// Provides: {"impl_202"}
// Dependencies: {}
impl < N : AstNode > Hash for AstPtr < N > { fn hash < H : Hasher > (& self , state : & mut H) { self . raw . hash (state) } }
};
}
