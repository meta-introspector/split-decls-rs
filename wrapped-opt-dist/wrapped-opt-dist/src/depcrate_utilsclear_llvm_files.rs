// Generated macro for clear_llvm_files (function)
macro_rules! Depcrate_utilsclear_llvm_files {
() => {
// Module: crate::utils
// Provides: {"clear_llvm_files"}
// Dependencies: {}
pub fn clear_llvm_files (env : & Environment) -> anyhow :: Result < () > { log :: info ! ("Clearing LLVM build files") ; delete_directory (& env . build_artifacts () . join ("llvm")) ? ; if env . build_artifacts () . join ("lld") . is_dir () { delete_directory (& env . build_artifacts () . join ("lld")) ? ; } Ok (()) }
};
}
