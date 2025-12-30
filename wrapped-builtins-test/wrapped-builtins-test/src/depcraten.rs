// Generated macro for N (const)
macro_rules! DepcrateN {
() => {
// Module: crate
// Provides: {"N"}
// Dependencies: {}
# [doc = " Sets the number of fuzz iterations run for most tests. In practice, the vast majority of bugs"] # [doc = " are caught by the edge case testers. Most of the remaining bugs triggered by more complex"] # [doc = " sequences are caught well within 10_000 fuzz iterations. For classes of algorithms like division"] # [doc = " that are vulnerable to rare edge cases, we want 1_000_000 iterations to be more confident. In"] # [doc = " practical CI, however, we only want to run the more strenuous test once to catch algorithmic"] # [doc = " level bugs, and run the 10_000 iteration test on most targets. Target-dependent bugs are likely"] # [doc = " to involve miscompilation and misconfiguration that is likely to break algorithms in quickly"] # [doc = " caught ways. We choose to configure `N = 1_000_000` iterations for `x86_64` targets (and if"] # [doc = " debug assertions are disabled. Tests without `--release` would take too long) which are likely"] # [doc = " to have fast hardware, and run `N = 10_000` for all other targets."] pub const N : u32 = if cfg ! (target_arch = "x86_64") && ! cfg ! (debug_assertions) { 1_000_000 } else { 10_000 } ;
};
}
