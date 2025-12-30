// Generated macro for gather_rustc_profiles (function)
macro_rules! Depcrate_traininggather_rustc_profiles {
() => {
// Module: crate::training
// Provides: {"gather_rustc_profiles"}
// Dependencies: {}
pub fn gather_rustc_profiles (env : & Environment , profile_root : & Utf8Path ,) -> anyhow :: Result < RustcPGOProfile > { log :: info ! ("Running benchmarks with PGO instrumented rustc") ; let profile_template = profile_root . join ("default_%m_%p.profraw") ; with_log_group ("Running benchmarks" , | | { rustc_benchmarks (env) . env ("LLVM_PROFILE_FILE" , profile_template . as_str ()) . run () . context ("Cannot gather rustc PGO profiles") }) ? ; let merged_profile = env . artifact_dir () . join ("rustc-pgo.profdata") ; log :: info ! ("Merging Rustc PGO profiles to {merged_profile}") ; let llvm_profdata = if env . build_llvm () { LlvmProfdata :: Target } else { LlvmProfdata :: Host } ; merge_llvm_profiles (env , & merged_profile , profile_root , llvm_profdata) ? ; log_profile_stats ("Rustc" , & merged_profile , profile_root) ? ; delete_directory (profile_root) ? ; Ok (RustcPGOProfile (merged_profile)) }
};
}
