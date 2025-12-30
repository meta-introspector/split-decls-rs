// Generated macro for get_llvm_version (function)
macro_rules! Depcrate_core_build_steps_llvmget_llvm_version {
() => {
// Module: crate::core::build_steps::llvm
// Provides: {"get_llvm_version"}
// Dependencies: {}
pub fn get_llvm_version (builder : & Builder < '_ > , llvm_config : & Path) -> String { command (llvm_config) . arg ("--version") . cached () . run_capture_stdout (builder) . stdout () . trim () . to_owned () }
};
}
