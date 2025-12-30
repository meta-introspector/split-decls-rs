// Generated macro for get_scenario (function)
macro_rules! Depcrate_scenarios_empty_resource_all_localesget_scenario {
() => {
// Module: crate::scenarios::empty_resource_all_locales
// Provides: {"get_scenario"}
// Dependencies: {}
# [doc = " Tests bundle generation with a queried value that is missing in all resources"] # [doc = " in all locales. Despite the fact that the value is missing, this should have no"] # [doc = " effect on the ability to generate a bundle and look up other present values."] pub fn get_scenario () -> Scenario { Scenario :: new ("empty_resource_all_locales" , vec ! [FileSource :: new ("browser" , "browser/{locale}/" , vec ! ["en-US" , "pl"]) , FileSource :: new ("empty" , "empty-resource/{locale}/" , vec ! ["en-US" , "pl"]) ,] , vec ! ["en-US" , "pl"] , vec ! ["browser/sanitize.ftl" , "empty/empty-all.ftl"] , queries ! [("history-section-label" , "History" , ExceptionalContext :: None) , ("empty-all" , "empty-all" , ExceptionalContext :: ValueMissingFromAllResources ,)] ,) }
};
}
