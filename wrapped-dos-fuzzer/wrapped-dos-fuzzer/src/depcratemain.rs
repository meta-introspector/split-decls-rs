// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let num_cpus = (num_cpus :: get () as f32 * 0.8) . ceil () as usize ; let Opts { retest , regressions , } = clap :: Parser :: parse () ; if retest { for pattern in io :: stdin () . lock () . lines () . flatten () { println ! ("Retesting: {}" , & pattern) ; let pattern = serde_json :: from_str (& pattern) . expect ("Couldn't deserialize pattern") ; match test_catch_unwind (& pattern) { Ok (res) => println ! ("score: {}" , res . score ()) , Err (()) => () , } } } else if regressions { let exit_code = regression_test () ; std :: process :: exit (exit_code) ; } else { fuzz (num_cpus) ; } }
};
}
