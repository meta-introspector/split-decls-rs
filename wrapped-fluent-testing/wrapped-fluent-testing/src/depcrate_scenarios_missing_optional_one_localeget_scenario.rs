// Generated macro for get_scenario (function)
macro_rules! Depcrate_scenarios_missing_optional_one_localeget_scenario {
() => {
// Module: crate::scenarios::missing_optional_one_locale
// Provides: {"get_scenario"}
// Dependencies: {}
# [doc = " Tests bundle generation with an optional resource that is missing from only the primary locale."] # [doc = " Since the resource is optional, we should only fallback to another locale for values in the missing"] # [doc = " optional resource. We should still be able to use the primary locale for other present values/resources."] pub fn get_scenario () -> Scenario { Scenario :: new ("missing_optional_one_locale" , vec ! [FileSource :: new ("browser" , "browser/{locale}/" , vec ! ["en-US" , "pl"]) , FileSource :: new ("missing" , "missing-resource/{locale}/" , vec ! ["en-US" , "pl"]) ,] , vec ! ["en-US" , "pl"] , vec ! ["browser/sanitize.ftl" . into () , "missing/missing-one.ftl" . to_resource_id (ResourceType :: Optional) ,] , queries ! [("history-section-label" , "History" , ExceptionalContext :: None) , ("missing-one" , "zaginiony" , ExceptionalContext :: OptionalResourceMissingFromLocale ,)] ,) }
};
}
