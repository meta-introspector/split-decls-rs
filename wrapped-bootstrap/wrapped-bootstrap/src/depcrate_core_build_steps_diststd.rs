// Generated macro for Std (struct)
macro_rules! Depcrate_core_build_steps_distStd {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"Std"}
// Dependencies: {}
# [doc = " Builds the standard library (`rust-std`) dist component for a given `target`."] # [doc = " This includes the standard library dynamic library file (e.g. .so/.dll), along with stdlib"] # [doc = " .rlibs."] # [doc = ""] # [doc = " Note that due to uplifting, we actually ship the stage 1 library"] # [doc = " (built using the stage1 compiler) even with a stage 2 dist, unless `full-bootstrap` is enabled."] # [derive (Debug , Clone , Hash , PartialEq , Eq)] pub struct Std { # [doc = " Compiler that will build the standard library."] pub build_compiler : Compiler , pub target : TargetSelection , }
};
}
