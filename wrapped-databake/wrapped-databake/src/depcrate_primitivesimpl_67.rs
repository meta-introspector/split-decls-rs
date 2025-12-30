// Generated macro for impl_67 (impl)
macro_rules! Depcrate_primitivesimpl_67 {
() => {
// Module: crate::primitives
// Provides: {"impl_67"}
// Dependencies: {}
impl < T > Bake for Option < T > where T : Bake , { fn bake (& self , ctx : & CrateEnv) -> TokenStream { match self { None => quote ! { None } , Some (t) => { let t = t . bake (ctx) ; quote ! { Some (# t) } } } } }
};
}
