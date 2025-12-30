// Generated macro for get_scenario (function)
macro_rules! Depcrate_scenarios_empty_resource_one_localeget_scenario {
() => {
// Module: crate::scenarios::empty_resource_one_locale
// Provides: {"get_scenario"}
// Dependencies: {}
# [doc = " Tests bundle generation with a queried value that is missing in only one resource"] # [doc = " in the primary locale. This should cause the bundle to fallback to another locale"] # [doc = " only for that value."] pub fn get_scenario () -> Scenario { Scenario :: new ("empty_resource_one_locale" , vec ! [FileSource :: new ("browser" , "browser/{locale}/" , vec ! ["en-US" , "pl"]) , FileSource :: new ("empty" , "empty-resource/{locale}/" , vec ! ["en-US" , "pl"]) ,] , vec ! ["en-US" , "pl"] , vec ! ["browser/sanitize.ftl" , "empty/empty-one.ftl"] , queries ! [("history-section-label" , "History" , ExceptionalContext :: None) , ("empty-one" , "pusty" , ExceptionalContext :: ValueMissingFromResource ,)] ,) }
};
}
