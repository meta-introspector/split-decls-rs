// Generated macro for get_scenario (function)
macro_rules! Depcrate_scenarios_missing_required_all_localesget_scenario {
() => {
// Module: crate::scenarios::missing_required_all_locales
// Provides: {"get_scenario"}
// Dependencies: {}
# [doc = " Tests bundle generation with a required resource that is missing from all locales."] # [doc = " Since the resource is required, we cannot generate a bundle because no solution exists."] # [doc = " Lookups for all values should fail, because no bundle will be generated."] pub fn get_scenario () -> Scenario { Scenario :: new ("missing_required_all_locales" , vec ! [FileSource :: new ("browser" , "browser/{locale}/" , vec ! ["en-US" , "pl"]) , FileSource :: new ("missing" , "missing-resource/{locale}/" , vec ! ["en-US" , "pl"]) ,] , vec ! ["en-US" , "pl"] , vec ! ["browser/sanitize.ftl" , "missing/missing-one.ftl" , "missing/missing-all.ftl" ,] , queries ! [("history-section-label" , "history-section-label" , ExceptionalContext :: None ,) , ("missing-one" , "missing-one" , ExceptionalContext :: RequiredResourceMissingFromLocale ,) , ("missing-all" , "missing-all" , ExceptionalContext :: RequiredResourceMissingFromAllLocales ,)] ,) }
};
}
