// Generated macro for passes_filter (function)
macro_rules! Depcrate_benchmarkpasses_filter {
() => {
// Module: crate::benchmark
// Provides: {"passes_filter"}
// Dependencies: {}
# [doc = " Tests if the name of the benchmark passes through the include and exclude filters."] # [doc = " Both filters can contain multiple comma-separated prefixes."] pub fn passes_filter (name : & str , exclude : & [String] , include : & [String]) -> bool { match (exclude , include) { (exclude , include) if ! exclude . is_empty () && ! include . is_empty () => { let included = include . iter () . any (| filter | name . starts_with (filter)) ; let excluded = exclude . iter () . any (| filter | name . starts_with (filter)) ; included && ! excluded } ([] , include) if ! include . is_empty () => { include . iter () . any (| filter | name . starts_with (filter)) } (exclude , []) if ! exclude . is_empty () => { ! exclude . iter () . any (| filter | name . starts_with (filter)) } ([] , []) => true , (_ , _) => unreachable ! () , } }
};
}
