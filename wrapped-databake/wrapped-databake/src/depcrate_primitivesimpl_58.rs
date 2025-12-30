// Generated macro for impl_58 (impl)
macro_rules! Depcrate_primitivesimpl_58 {
() => {
// Module: crate::primitives
// Provides: {"impl_58"}
// Dependencies: {}
impl < T > Bake for & T where T : Bake , { fn bake (& self , ctx : & CrateEnv) -> TokenStream { let t = < T as Bake > :: bake (* self , ctx) ; quote ! { &# t } } }
};
}
