// Generated macro for CheckBasis (enum)
macro_rules! Depcrate_run_cfgCheckBasis {
() => {
// Module: crate::run_cfg
// Provides: {"CheckBasis"}
// Dependencies: {}
# [doc = " Possible items to test against"] # [derive (Clone , Debug , PartialEq , Eq)] pub enum CheckBasis { # [doc = " Check against Musl's math sources."] Musl , # [doc = " Check against infinite precision (MPFR)."] Mpfr , # [doc = " Benchmarks or other times when this is not relevant."] None , }
};
}
