// Generated macro for get_scenario (function)
macro_rules! Depcrate_scenarios_missing_optional_all_localesget_scenario {
() => {
// Module: crate::scenarios::missing_optional_all_locales
// Provides: {"get_scenario"}
// Dependencies: {}
# [doc = " Tests bundle generation with an optional resource that is missing from all locales."] # [doc = " Since the resource is optional, we should still be able to generate a bundle and"] # [doc = " look up other present values, but we will fail to look up a value from the missing resource."] pub fn get_scenario () -> Scenario { Scenario :: new ("missing_optional_all_locales" , vec ! [FileSource :: new ("browser" , "browser/{locale}/" , vec ! ["en-US" , "pl"]) , FileSource :: new ("missing" , "missing-resource/{locale}/" , vec ! ["en-US" , "pl"]) ,] , vec ! ["en-US" , "pl"] , vec ! ["browser/sanitize.ftl" . into () , "missing/missing-one.ftl" . to_resource_id (ResourceType :: Optional) , "missing/missing-all.ftl" . to_resource_id (ResourceType :: Optional) ,] , queries ! [("history-section-label" , "History" , ExceptionalContext :: None) , ("missing-one" , "zaginiony" , ExceptionalContext :: OptionalResourceMissingFromLocale ,) , ("missing-all" , "missing-all" , ExceptionalContext :: OptionalResourceMissingFromAllLocales ,)] ,) }
};
}
