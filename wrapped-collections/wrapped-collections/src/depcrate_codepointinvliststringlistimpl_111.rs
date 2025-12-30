// Generated macro for impl_111 (impl)
macro_rules! Depcrate_codepointinvliststringlistimpl_111 {
() => {
// Module: crate::codepointinvliststringlist
// Provides: {"impl_111"}
// Dependencies: {}
# [cfg (feature = "databake")] impl databake :: Bake for CodePointInversionListAndStringList < '_ > { fn bake (& self , env : & databake :: CrateEnv) -> databake :: TokenStream { env . insert ("icu_collections") ; let cp_inv_list = self . cp_inv_list . bake (env) ; let str_list = self . str_list . bake (env) ; databake :: quote ! { icu_collections :: codepointinvliststringlist :: CodePointInversionListAndStringList :: from_parts_unchecked (# cp_inv_list , # str_list) } } }
};
}
