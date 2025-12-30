// Generated macro for read_icount_results (function)
macro_rules! Depcrateread_icount_results {
() => {
// Module: crate
// Provides: {"read_icount_results"}
// Dependencies: {}
# [doc = " Reads the (benchmark, instruction count) pairs from previous CSV output"] fn read_icount_results (path : & Path) -> anyhow :: Result < HashMap < String , u64 > > { let file = File :: open (path) . context (format ! ("CSV file for comparison not found: {}" , path . display ())) ? ; let mut measurements = HashMap :: new () ; for line in BufReader :: new (file) . lines () { let line = line . context ("Unable to read results from CSV file") ? ; let line = line . trim () ; let mut parts = line . split (',') ; measurements . insert (parts . next () . ok_or (anyhow :: anyhow ! ("CSV is wrongly formatted")) ? . to_string () , parts . next () . ok_or (anyhow :: anyhow ! ("CSV is wrongly formatted")) ? . parse () . context ("Unable to parse instruction count from CSV") ? ,) ; } Ok (measurements) }
};
}
