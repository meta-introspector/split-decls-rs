// Generated macro for impl_83 (impl)
macro_rules! Depcrate_providerimpl_83 {
() => {
// Module: crate::provider
// Provides: {"impl_83"}
// Dependencies: {}
# [cfg (feature = "datagen")] impl < 'a , V > databake :: Bake for & 'a PluralElementsPackedULE < V > where & 'a V : databake :: Bake , V : VarULE + ? Sized , { fn bake (& self , ctx : & databake :: CrateEnv) -> databake :: TokenStream { ctx . insert ("icu_plurals") ; let bytes = (& self . bytes) . bake (ctx) ; databake :: quote ! { unsafe { icu_plurals :: provider :: PluralElementsPackedULE :: from_bytes_unchecked (# bytes) } } } }
};
}
