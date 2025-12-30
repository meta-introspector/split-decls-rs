// Generated macro for get_llvm_version_major (function)
macro_rules! Depcrate_core_build_steps_llvmget_llvm_version_major {
() => {
// Module: crate::core::build_steps::llvm
// Provides: {"get_llvm_version_major"}
// Dependencies: {}
pub fn get_llvm_version_major (builder : & Builder < '_ > , llvm_config : & Path) -> u8 { let version = get_llvm_version (builder , llvm_config) ; let major_str = version . split_once ('.') . expect ("Failed to parse LLVM version") . 0 ; major_str . parse () . unwrap () }
};
}
