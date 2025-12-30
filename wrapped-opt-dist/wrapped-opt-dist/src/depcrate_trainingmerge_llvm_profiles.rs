// Generated macro for merge_llvm_profiles (function)
macro_rules! Depcrate_trainingmerge_llvm_profiles {
() => {
// Module: crate::training
// Provides: {"merge_llvm_profiles"}
// Dependencies: {}
fn merge_llvm_profiles (env : & Environment , merged_path : & Utf8Path , profile_dir : & Utf8Path , profdata : LlvmProfdata ,) -> anyhow :: Result < () > { let llvm_profdata = match profdata { LlvmProfdata :: Host => { env . host_llvm_dir () . join (format ! ("bin/llvm-profdata{}" , executable_extension ())) } LlvmProfdata :: Target => env . build_artifacts () . join ("llvm") . join ("build") . join (format ! ("bin/llvm-profdata{}" , executable_extension ())) , } ; cmd (& [llvm_profdata . as_str () , "merge" , "-o" , merged_path . as_str () , profile_dir . as_str ()]) . run () . context ("Cannot merge LLVM profiles") ? ; Ok (()) }
};
}
