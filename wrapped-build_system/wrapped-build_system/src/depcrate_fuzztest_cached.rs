// Generated macro for test_cached (function)
macro_rules! Depcrate_fuzztest_cached {
() => {
// Module: crate::fuzz
// Provides: {"test_cached"}
// Dependencies: {}
# [doc = " Tests a file with a cached LLVM result. Used for reduction, when it is known"] # [doc = " that a given transformation should not change the execution result."] fn test_cached (source_file : & Path , remove_tmps : bool , cache : & mut ResultCache ,) -> Result < Result < () , std :: path :: PathBuf > , String > { let gcc_res = release_gcc (source_file) ? ; if cache . is_none () { * cache = Some ((debug_llvm (source_file) ? , gcc_res . clone ())) ; } let (llvm_res , old_gcc) = cache . as_ref () . unwrap () ; if * llvm_res != gcc_res && gcc_res == * old_gcc { Ok (Err (source_file . to_path_buf ())) } else { if remove_tmps { std :: fs :: remove_file (source_file) . map_err (| err | format ! ("{err:?}")) ? ; } Ok (Ok (())) } }
};
}
