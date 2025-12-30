// Generated macro for impl_70 (impl)
macro_rules! Depcrate_primitivesimpl_70 {
() => {
// Module: crate::primitives
// Provides: {"impl_70"}
// Dependencies: {}
impl < T , E > Bake for Result < T , E > where T : Bake , E : Bake , { fn bake (& self , ctx : & CrateEnv) -> TokenStream { match self { Ok (ok) => { let ok = ok . bake (ctx) ; quote ! { Ok (# ok) } } Err (e) => { let e = e . bake (ctx) ; quote ! { Err (# e) } } } } }
};
}
