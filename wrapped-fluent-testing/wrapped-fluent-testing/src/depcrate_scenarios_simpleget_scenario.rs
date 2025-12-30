// Generated macro for get_scenario (function)
macro_rules! Depcrate_scenarios_simpleget_scenario {
() => {
// Module: crate::scenarios::simple
// Provides: {"get_scenario"}
// Dependencies: {}
pub fn get_scenario () -> Scenario { Scenario :: new ("simple" , vec ! [FileSource :: new ("browser" , "browser/{locale}/" , vec ! ["en-US" , "pl"] ,)] , vec ! ["en-US"] , vec ! ["browser/sanitize.ftl"] , queries ! [("history-section-label" , "History")] ,) }
};
}
