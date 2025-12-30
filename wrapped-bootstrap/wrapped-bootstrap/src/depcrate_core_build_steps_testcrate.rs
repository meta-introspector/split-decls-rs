// Generated macro for Crate (struct)
macro_rules! Depcrate_core_build_steps_testCrate {
() => {
// Module: crate::core::build_steps::test
// Provides: {"Crate"}
// Dependencies: {}
# [doc = " Runs `cargo test` for standard library crates."] # [doc = ""] # [doc = " (Also used internally to run `cargo test` for compiler crates.)"] # [doc = ""] # [doc = " FIXME(Zalathar): Try to split this into two separate steps: a user-visible"] # [doc = " step for testing standard library crates, and an internal step used for both"] # [doc = " library crates and compiler crates."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct Crate { # [doc = " The compiler that will *build* libstd or rustc in test mode."] build_compiler : Compiler , target : TargetSelection , mode : Mode , crates : Vec < String > , }
};
}
