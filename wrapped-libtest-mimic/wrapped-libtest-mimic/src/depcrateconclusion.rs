// Generated macro for Conclusion (struct)
macro_rules! DepcrateConclusion {
() => {
// Module: crate
// Provides: {"Conclusion"}
// Dependencies: {}
# [doc = " Contains information about the entire test run. Is returned by[`run`]."] # [doc = ""] # [doc = " This type is marked as `#[must_use]`. Usually, you just call"] # [doc = " [`exit()`][Conclusion::exit] on the result of `run` to exit the application"] # [doc = " with the correct exit code. But you can also store this value and inspect"] # [doc = " its data."] # [derive (Clone , Debug , PartialEq , Eq)] # [must_use = "Call `exit()` or `exit_if_failed()` to set the correct return code"] pub struct Conclusion { # [doc = " Number of tests and benchmarks that were filtered out (either by the"] # [doc = " filter-in pattern or by `--skip` arguments)."] pub num_filtered_out : u64 , # [doc = " Number of passed tests."] pub num_passed : u64 , # [doc = " Number of failed tests and benchmarks."] pub num_failed : u64 , # [doc = " Number of ignored tests and benchmarks."] pub num_ignored : u64 , # [doc = " Number of benchmarks that successfully ran."] pub num_measured : u64 , }
};
}
