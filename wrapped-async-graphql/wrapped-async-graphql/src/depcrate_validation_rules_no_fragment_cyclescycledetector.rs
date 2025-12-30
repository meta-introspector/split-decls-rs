// Generated macro for CycleDetector (struct)
macro_rules! Depcrate_validation_rules_no_fragment_cyclesCycleDetector {
() => {
// Module: crate::validation::rules::no_fragment_cycles
// Provides: {"CycleDetector"}
// Dependencies: {}
struct CycleDetector < 'a > { visited : HashSet < & 'a str > , spreads : & 'a HashMap < & 'a str , Vec < (& 'a str , Pos) > > , path_indices : HashMap < & 'a str , usize > , errors : Vec < RuleError > , }
};
}
