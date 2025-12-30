// Generated macro for impl_717 (impl)
macro_rules! Depcrate_provider_pattern_runtime_patternimpl_717 {
() => {
// Module: crate::provider::pattern::runtime::pattern
// Provides: {"impl_717"}
// Dependencies: {}
# [cfg (feature = "datagen")] impl databake :: Bake for PatternMetadata { fn bake (& self , ctx : & databake :: CrateEnv) -> databake :: TokenStream { ctx . insert ("icu_datetime") ; let time_granularity = databake :: Bake :: bake (& self . time_granularity () , ctx) ; databake :: quote ! { icu_datetime :: provider :: pattern :: runtime :: PatternMetadata :: from_time_granularity (# time_granularity) } } }
};
}
