// Generated macro for CrateLibrustc (struct)
macro_rules! Depcrate_core_build_steps_testCrateLibrustc {
() => {
// Module: crate::core::build_steps::test
// Provides: {"CrateLibrustc"}
// Dependencies: {}
# [doc = " Runs `cargo test` for the compiler crates in `compiler/`."] # [doc = ""] # [doc = " (This step does not test `rustc_codegen_cranelift` or `rustc_codegen_gcc`,"] # [doc = " which have their own separate test steps.)"] # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct CrateLibrustc { # [doc = " The compiler that will run unit tests and doctests on the in-tree rustc source."] build_compiler : Compiler , target : TargetSelection , crates : Vec < String > , }
};
}
