// Generated macro for RustcPrivateCompilers (struct)
macro_rules! Depcrate_core_build_steps_toolRustcPrivateCompilers {
() => {
// Module: crate::core::build_steps::tool
// Provides: {"RustcPrivateCompilers"}
// Dependencies: {}
# [doc = " Represents which compilers are involved in the compilation of a tool"] # [doc = " that depends on compiler internals (`rustc_private`)."] # [doc = " Their compilation looks like this:"] # [doc = ""] # [doc = " - `build_compiler` (stage N-1) builds `target_compiler` (stage N) to produce .rlibs"] # [doc = "     - These .rlibs are copied into the sysroot of `build_compiler`"] # [doc = " - `build_compiler` (stage N-1) builds `<tool>` (stage N)"] # [doc = "     - `<tool>` links to .rlibs from `target_compiler`"] # [doc = ""] # [doc = " Eventually, this could also be used for .rmetas and check builds, but so far we only deal with"] # [doc = " normal builds here."] # [derive (Copy , Clone , Debug , Hash , PartialEq , Eq)] pub struct RustcPrivateCompilers { # [doc = " Compiler that builds the tool and that builds `target_compiler`."] build_compiler : Compiler , # [doc = " Compiler to which .rlib artifacts the tool links to."] # [doc = " The host target of this compiler corresponds to the target of the tool."] target_compiler : Compiler , }
};
}
