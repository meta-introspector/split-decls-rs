// Generated macro for Cargotest (struct)
macro_rules! Depcrate_core_build_steps_testCargotest {
() => {
// Module: crate::core::build_steps::test
// Provides: {"Cargotest"}
// Dependencies: {}
# [doc = " Builds cargo and then runs the `src/tools/cargotest` tool, which checks out"] # [doc = " some representative crate repositories and runs `cargo test` on them, in"] # [doc = " order to test cargo."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct Cargotest { build_compiler : Compiler , host : TargetSelection , }
};
}
