// Generated macro for impl_79 (impl)
macro_rules! Depcrate_codepointinvlist_cpinvlistimpl_79 {
() => {
// Module: crate::codepointinvlist::cpinvlist
// Provides: {"impl_79"}
// Dependencies: {}
# [cfg (feature = "databake")] impl databake :: Bake for CodePointInversionList < '_ > { fn bake (& self , env : & databake :: CrateEnv) -> databake :: TokenStream { env . insert ("icu_collections") ; let inv_list = self . inv_list . bake (env) ; let size = self . size . bake (env) ; databake :: quote ! { unsafe { # [allow (unused_unsafe)] icu_collections :: codepointinvlist :: CodePointInversionList :: from_parts_unchecked (# inv_list , # size) } } } }
};
}
