// Generated macro for State (struct)
macro_rules! Depcrate_throughputState {
() => {
// Module: crate::throughput
// Provides: {"State"}
// Dependencies: {}
# [derive (Clone , Eq , PartialEq , Ord , PartialOrd , Debug)] struct State { observed : Duration , last_value : progress :: Step , elapsed_values : VecDeque < (Duration , progress :: Step) > , last_update_duration : Duration , precomputed_throughput : Option < progress :: Step > , }
};
}
