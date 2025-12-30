// Generated macro for check_version (function)
macro_rules! Depcratecheck_version {
() => {
// Module: crate
// Provides: {"check_version"}
// Dependencies: {}
# [doc = " Checks that the instrumented binary uses the same profiling data format as"] # [doc = " the LLVM profiling runtime."] fn check_version () { let version = unsafe { __llvm_profile_get_version () & ! VARIANT_MASKS_ALL } ; assert_eq ! (version , INSTR_PROF_RAW_VERSION , "Runtime and instrumentation version mismatch") ; }
};
}
