// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let github_events = format ! ("[{}]" , std :: iter :: repeat (GITHUB_EVENTS) . take (100) . collect ::< Vec < _ >> () . join (",")) ; run_benchmark_group (| group | { group . register_benchmark ("nom-json" , | | | | parse_json (& github_events) . unwrap ()) ; }) ; }
};
}
