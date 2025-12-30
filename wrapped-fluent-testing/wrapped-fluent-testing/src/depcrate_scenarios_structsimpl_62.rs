// Generated macro for impl_62 (impl)
macro_rules! Depcrate_scenarios_structsimpl_62 {
() => {
// Module: crate::scenarios::structs
// Provides: {"impl_62"}
// Dependencies: {}
impl ExceptionalContext { # [doc = " This is a query for a value in a missing required resource."] pub fn missing_required_resource (self) -> bool { matches ! (self , Self :: RequiredResourceMissingFromLocale | Self :: RequiredResourceMissingFromAllLocales ,) } # [doc = " This query should cause a format error to be appended to the errors Vec."] pub fn causes_reported_format_error (self) -> bool { matches ! (self , Self :: ValueMissingFromResource | Self :: ValueMissingFromAllResources | Self :: OptionalResourceMissingFromLocale | Self :: OptionalResourceMissingFromAllLocales | Self :: RequiredResourceMissingFromAllLocales ,) } # [doc = " This query should cause a failed value lookup."] pub fn causes_failed_value_lookup (self) -> bool { matches ! (self , Self :: ValueMissingFromAllResources | Self :: OptionalResourceMissingFromAllLocales | Self :: RequiredResourceMissingFromAllLocales ,) } # [doc = " This query should result in no bundles being generated."] pub fn blocks_bundle_generation (self) -> bool { matches ! (self , Self :: RequiredResourceMissingFromAllLocales ,) } }
};
}
