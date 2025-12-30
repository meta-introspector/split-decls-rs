// Generated macro for impl_297 (impl)
macro_rules! Depcrate_dimension_provider_units_display_namesimpl_297 {
() => {
// Module: crate::dimension::provider::units::display_names
// Provides: {"impl_297"}
// Dependencies: {}
# [cfg (feature = "datagen")] impl databake :: Bake for UnitsDisplayNames < '_ > { fn bake (& self , ctx : & databake :: CrateEnv) -> databake :: TokenStream { use zerovec :: ule :: VarULE ; ctx . insert ("icu_experimental::dimension::provider::units::display_names") ; let bytes = self . patterns . elements . as_bytes () . bake (ctx) ; databake :: quote ! { unsafe { icu_experimental :: dimension :: provider :: units :: display_names :: UnitsDisplayNames :: from_bytes_unchecked (# bytes) } } } }
};
}
