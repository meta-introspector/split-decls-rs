// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: { dump_error , init } ; # [test] fn test_init () { init () ; } # [test] fn test_dump () { unsafe { dump_error () ; } } # [cfg (not (feature = "fips"))] # [test] fn test_fips () { assert ! ({ crate :: try_fips_mode () . is_err () }) ; } # [test] # [cfg (feature = "fips")] fn test_fips () { # [cfg (not (feature = "asan"))] crate :: fips_mode () ; if aws_lc :: CFG_CPU_JITTER_ENTROPY () { crate :: fips_cpu_jitter_entropy () ; } } }
};
}
