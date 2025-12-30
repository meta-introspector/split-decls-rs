// Generated macro for impl_32 (impl)
macro_rules! Depcrate_converterimpl_32 {
() => {
// Module: crate::converter
// Provides: {"impl_32"}
// Dependencies: {}
impl < T > Bake for AsStaticStr < T > where T : AsRef < str > , { fn bake (& self , _ctx : & CrateEnv) -> TokenStream { let value = & self . 0 . as_ref () ; quote ! (# value) } }
};
}
