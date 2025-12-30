// Generated macro for BenchYamlOutput (struct)
macro_rules! DepcrateBenchYamlOutput {
() => {
// Module: crate
// Provides: {"BenchYamlOutput"}
// Dependencies: {}
# [doc = " Ourput of running `run_bench` on a given parser."] # [derive (Serialize , Deserialize)] struct BenchYamlOutput { # [doc = " The name of the parser."] parser : String , # [doc = " The file taken as input."] input : String , # [doc = " Average parsing time (ns)."] average : u64 , # [doc = " Shortest parsing time (ns)."] min : u64 , # [doc = " Longest parsing time (ns)."] max : u64 , # [doc = " 95th percentile of parsing times (ns)."] percentile95 : u64 , # [doc = " Number of iterations."] iterations : u64 , # [doc = " Parsing times for each run."] times : Vec < u64 > , }
};
}
