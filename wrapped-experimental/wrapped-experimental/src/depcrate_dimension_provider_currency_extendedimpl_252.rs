// Generated macro for impl_252 (impl)
macro_rules! Depcrate_dimension_provider_currency_extendedimpl_252 {
() => {
// Module: crate::dimension::provider::currency::extended
// Provides: {"impl_252"}
// Dependencies: {}
# [cfg (feature = "datagen")] impl databake :: Bake for CurrencyExtendedData < '_ > { fn bake (& self , ctx : & databake :: CrateEnv) -> databake :: TokenStream { use zerovec :: ule :: VarULE ; ctx . insert ("icu_experimental::dimension::provider::currency") ; let bytes = self . display_names . elements . as_bytes () . bake (ctx) ; databake :: quote ! { unsafe { icu_experimental :: dimension :: provider :: currency :: extended :: CurrencyExtendedData :: from_bytes_unchecked (# bytes) } } } }
};
}
