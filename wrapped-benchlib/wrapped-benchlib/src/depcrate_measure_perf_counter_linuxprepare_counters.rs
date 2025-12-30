// Generated macro for prepare_counters (function)
macro_rules! Depcrate_measure_perf_counter_linuxprepare_counters {
() => {
// Module: crate::measure::perf_counter::linux
// Provides: {"prepare_counters"}
// Dependencies: {}
fn prepare_counters (group : & mut Group) -> anyhow :: Result < Counters > { let mut add_event = | event : Hardware | match Builder :: new () . group (group) . kind (event) . build () { Ok (counter) => Some (counter) , Err (error) => { log :: warn ! ("Could not add counter {:?}: {:?}. Maybe the CPU doesn't support it?" , event , error) ; None } } ; let cycles = add_event (Hardware :: CPU_CYCLES) ; let instructions = add_event (Hardware :: INSTRUCTIONS) ; let branch_misses = add_event (Hardware :: BRANCH_MISSES) ; let cache_misses = add_event (Hardware :: CACHE_MISSES) ; let cache_references = add_event (Hardware :: CACHE_REFERENCES) ; Ok (Counters { cycles , instructions , branch_misses , cache_misses , cache_references , }) }
};
}
