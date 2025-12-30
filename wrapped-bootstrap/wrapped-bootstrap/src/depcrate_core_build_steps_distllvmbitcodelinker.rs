// Generated macro for LlvmBitcodeLinker (struct)
macro_rules! Depcrate_core_build_steps_distLlvmBitcodeLinker {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"LlvmBitcodeLinker"}
// Dependencies: {}
# [doc = " Distributes the `llvm-bitcode-linker` tool so that it can be used by a compiler whose host"] # [doc = " is `target`."] # [derive (Debug , Clone , Hash , PartialEq , Eq)] pub struct LlvmBitcodeLinker { # [doc = " The linker will be compiled by this compiler."] pub build_compiler : Compiler , # [doc = " The linker will by usable by rustc on this host."] pub target : TargetSelection , }
};
}
