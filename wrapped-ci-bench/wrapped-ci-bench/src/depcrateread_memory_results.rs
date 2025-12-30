// Generated macro for read_memory_results (function)
macro_rules! Depcrateread_memory_results {
() => {
// Module: crate
// Provides: {"read_memory_results"}
// Dependencies: {}
# [doc = " Reads the (benchmark, instruction count) pairs from previous CSV output"] fn read_memory_results (path : & Path) -> anyhow :: Result < HashMap < String , MemoryDetails > > { let file = File :: open (path) . context (format ! ("CSV file for comparison not found: {}" , path . display ())) ? ; let mut measurements = HashMap :: new () ; for line in BufReader :: new (file) . lines () { let line = line . context ("Unable to read results from CSV file") ? ; let line = line . trim () ; let mut parts = line . split (',') ; measurements . insert (parts . next () . ok_or (anyhow :: anyhow ! ("CSV is wrongly formatted")) ? . to_string () , MemoryDetails { heap_total_bytes : parts . next () . ok_or (anyhow :: anyhow ! ("CSV is wrongly formatted")) ? . parse () . context ("Unable to parse heap total bytes from CSV") ? , heap_total_blocks : parts . next () . ok_or (anyhow :: anyhow ! ("CSV is wrongly formatted")) ? . parse () . context ("Unable to parse heap total blocks from CSV") ? , heap_peak_bytes : parts . next () . ok_or (anyhow :: anyhow ! ("CSV is wrongly formatted")) ? . parse () . context ("Unable to parse heap peak bytes from CSV") ? , heap_peak_blocks : parts . next () . ok_or (anyhow :: anyhow ! ("CSV is wrongly formatted")) ? . parse () . context ("Unable to parse heap peak blocks from CSV") ? , } ,) ; } Ok (measurements) }
};
}
