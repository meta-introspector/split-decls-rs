// Generated macro for test (module)
macro_rules! Depcratetest {
() => {
// Module: crate
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: generate_rand_max_num_actions ; # [test] fn test_rand () { let mut nums : Vec < _ > = (0 .. 10_000) . map (generate_rand_max_num_actions) . collect () ; let min = nums . iter () . min () . copied () . unwrap () ; let max = nums . iter () . max () . copied () . unwrap () ; let mean = nums . iter () . sum :: < usize > () / nums . len () ; nums . sort () ; let median = nums [(nums . len () / 2) - 1] ; println ! ("{min} <> {mean} | {median} <> {max}") ; } }
};
}
