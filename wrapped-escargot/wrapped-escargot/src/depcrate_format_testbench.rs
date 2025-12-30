// Generated macro for Bench (struct)
macro_rules! Depcrate_format_testBench {
() => {
// Module: crate::format::test
// Provides: {"Bench"}
// Dependencies: {}
# [doc = " Benchmark event."] # [derive (Serialize , Deserialize , Clone , Debug , Eq , PartialEq)] # [non_exhaustive] pub struct Bench { # [doc = " Benchmark name."] pub name : String , # [doc = " Median performance."] pub median : usize , # [doc = " Deviation from median."] pub deviation : usize , # [doc = " Mb/s"] pub mib_per_second : Option < usize > , }
};
}
