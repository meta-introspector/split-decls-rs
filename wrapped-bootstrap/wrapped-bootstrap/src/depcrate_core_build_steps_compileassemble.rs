// Generated macro for Assemble (struct)
macro_rules! Depcrate_core_build_steps_compileAssemble {
() => {
// Module: crate::core::build_steps::compile
// Provides: {"Assemble"}
// Dependencies: {}
# [doc = " Prepare a compiler sysroot."] # [doc = ""] # [doc = " The sysroot may contain various things useful for running the compiler, like linkers and"] # [doc = " linker wrappers (LLD, LLVM bitcode linker, etc.)."] # [doc = ""] # [doc = " This will assemble a compiler in `build/$target/stage$stage`."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct Assemble { # [doc = " The compiler which we will produce in this step. Assemble itself will"] # [doc = " take care of ensuring that the necessary prerequisites to do so exist,"] # [doc = " that is, this can be e.g. a stage2 compiler and Assemble will build"] # [doc = " the previous stages for you."] pub target_compiler : Compiler , }
};
}
