// Generated macro for RustcDev (struct)
macro_rules! Depcrate_core_build_steps_distRustcDev {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"RustcDev"}
// Dependencies: {}
# [doc = " Tarball containing the compiler that gets downloaded and used by"] # [doc = " `rust.download-rustc`."] # [doc = ""] # [doc = " (Don't confuse this with [`RustDev`], without the `c`!)"] # [derive (Debug , Clone , Hash , PartialEq , Eq)] pub struct RustcDev { # [doc = " The compiler that will build rustc which will be shipped in this component."] build_compiler : Compiler , target : TargetSelection , }
};
}
