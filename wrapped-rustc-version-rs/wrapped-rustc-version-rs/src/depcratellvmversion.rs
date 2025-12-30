// Generated macro for LlvmVersion (struct)
macro_rules! DepcrateLlvmVersion {
() => {
// Module: crate
// Provides: {"LlvmVersion"}
// Dependencies: {}
# [doc = " LLVM version"] # [doc = ""] # [doc = " LLVM's version numbering scheme is not semver compatible until version 4.0"] # [doc = ""] # [doc = " rustc [just prints the major and minor versions], so other parts of the version are not included."] # [doc = ""] # [doc = " [just prints the major and minor versions]: https://github.com/rust-lang/rust/blob/b5c9e2448c9ace53ad5c11585803894651b18b0a/compiler/rustc_codegen_llvm/src/llvm_util.rs#L173-L178"] # [derive (Clone , Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] pub struct LlvmVersion { # [doc = " Major version"] pub major : u64 , # [doc = " Minor version"] pub minor : u64 , }
};
}
