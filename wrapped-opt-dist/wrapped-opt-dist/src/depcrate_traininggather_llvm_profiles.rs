// Generated macro for gather_llvm_profiles (function)
macro_rules! Depcrate_traininggather_llvm_profiles {
() => {
// Module: crate::training
// Provides: {"gather_llvm_profiles"}
// Dependencies: {}
pub fn gather_llvm_profiles (env : & Environment , profile_root : & Utf8Path ,) -> anyhow :: Result < LlvmPGOProfile > { log :: info ! ("Running benchmarks with PGO instrumented LLVM") ; with_log_group ("Running benchmarks" , | | { llvm_benchmarks (env) . run () . context ("Cannot gather LLVM PGO profiles") }) ? ; let merged_profile = env . artifact_dir () . join ("llvm-pgo.profdata") ; log :: info ! ("Merging LLVM PGO profiles to {merged_profile}") ; merge_llvm_profiles (env , & merged_profile , profile_root , LlvmProfdata :: Host) ? ; log_profile_stats ("LLVM" , & merged_profile , profile_root) ? ; delete_directory (profile_root) ? ; Ok (LlvmPGOProfile (merged_profile)) }
};
}
