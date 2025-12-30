// Generated macro for Opt (struct)
macro_rules! DepcrateOpt {
() => {
// Module: crate
// Provides: {"Opt"}
// Dependencies: {}
# [derive (Parser , Debug)] struct Opt { # [arg (required_unless_present = "dir")] file_prefix : Vec < PathBuf > , # [doc = " all event trace files in dir will be merged to one chrome_profiler.json file"] # [arg (long = "dir")] dir : Option < PathBuf > , # [doc = " collapse threads without overlapping events"] # [arg (long = "collapse-threads")] collapse_threads : bool , # [doc = " filter out events with shorter duration (in microseconds)"] # [arg (long = "minimum-duration")] minimum_duration : Option < u128 > , }
};
}
