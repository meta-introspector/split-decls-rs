// Generated macro for get_scenario (function)
macro_rules! Depcrate_scenarios_missing_required_one_localeget_scenario {
() => {
// Module: crate::scenarios::missing_required_one_locale
// Provides: {"get_scenario"}
// Dependencies: {}
# [doc = " Tests bundle generation with a required resource that is missing from only the primary locale."] # [doc = " Since the resource is required, we should only fallback entirely to the next locale for all resources."] pub fn get_scenario () -> Scenario { Scenario :: new ("missing_required_one_locale" , vec ! [FileSource :: new ("browser" , "browser/{locale}/" , vec ! ["en-US" , "pl"]) , FileSource :: new ("missing" , "missing-resource/{locale}/" , vec ! ["en-US" , "pl"]) ,] , vec ! ["en-US" , "pl"] , vec ! ["browser/sanitize.ftl" , "missing/missing-one.ftl"] , queries ! [("history-section-label" , "Historia" , ExceptionalContext :: None) , ("missing-one" , "zaginiony" , ExceptionalContext :: RequiredResourceMissingFromLocale ,)] ,) }
};
}
