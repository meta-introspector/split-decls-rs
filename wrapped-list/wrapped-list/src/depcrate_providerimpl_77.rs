// Generated macro for impl_77 (impl)
macro_rules! Depcrate_providerimpl_77 {
() => {
// Module: crate::provider
// Provides: {"impl_77"}
// Dependencies: {}
# [cfg (feature = "datagen")] impl databake :: Bake for ListJoinerPattern < '_ > { fn bake (& self , env : & databake :: CrateEnv) -> databake :: TokenStream { env . insert ("icu_list") ; let string = self . string . bake (env) ; let index_1 = self . index_1 . bake (env) ; databake :: quote ! { icu_list :: provider :: ListJoinerPattern :: from_parts (# string , # index_1) } } }
};
}
