// Generated macro for impl_2923 (impl)
macro_rules! Depcrate_ctxhashimpl_2923 {
() => {
// Module: crate::ctxhash
// Provides: {"impl_2923"}
// Dependencies: {}
impl < V : Eq + Hash > CtxHash < V > for NullCtx { fn ctx_hash < H : Hasher > (& self , state : & mut H , value : & V) { value . hash (state) ; } }
};
}
