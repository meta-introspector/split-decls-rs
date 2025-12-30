// Generated macro for LdFlags (struct)
macro_rules! Depcrate_core_build_steps_llvmLdFlags {
() => {
// Module: crate::core::build_steps::llvm
// Provides: {"LdFlags"}
// Dependencies: {}
# [doc = " Linker flags to pass to LLVM's CMake invocation."] # [derive (Debug , Clone , Default)] struct LdFlags { # [doc = " CMAKE_EXE_LINKER_FLAGS"] exe : OsString , # [doc = " CMAKE_SHARED_LINKER_FLAGS"] shared : OsString , # [doc = " CMAKE_MODULE_LINKER_FLAGS"] module : OsString , }
};
}
