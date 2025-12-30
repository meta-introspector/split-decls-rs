// Generated macro for ExceptionalContext (enum)
macro_rules! Depcrate_scenarios_structsExceptionalContext {
() => {
// Module: crate::scenarios::structs
// Provides: {"ExceptionalContext"}
// Dependencies: {}
# [derive (Clone , Copy , Debug)] pub enum ExceptionalContext { # [doc = " There is no exceptional context for this query (happy path)."] None , # [doc = " A value is missing from a resource and should cause a fallback."] ValueMissingFromResource , # [doc = " A value is missing from all resources in all locales."] ValueMissingFromAllResources , # [doc = " An optional resource is missing from the top locale."] OptionalResourceMissingFromLocale , # [doc = " An optional resource is missing from all locales."] OptionalResourceMissingFromAllLocales , # [doc = " A required resource is missing from the top locale."] RequiredResourceMissingFromLocale , # [doc = " A required resource is missing from all locales."] RequiredResourceMissingFromAllLocales , }
};
}
