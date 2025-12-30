// Generated macro for build_rustfmt_from_src (function)
macro_rules! Depcratebuild_rustfmt_from_src {
() => {
// Module: crate
// Provides: {"build_rustfmt_from_src"}
// Dependencies: {}
# [doc = " Obtains the ld_lib path and then builds rustfmt from source"] # [doc = " If that operation succeeds, the source is then copied to the output path specified"] pub fn build_rustfmt_from_src (binary_path : PathBuf , dir : & Path ,) -> Result < RustfmtRunner , CheckDiffError > { let ld_lib_path = get_ld_library_path (& dir) ? ; info ! ("Building rustfmt from source") ; let Ok (_) = Command :: new ("cargo") . current_dir (dir) . args (["build" , "-q" , "--release" , "--bin" , "rustfmt"]) . output () else { return Err (CheckDiffError :: FailedSourceBuild ("Error building rustfmt from source" ,)) ; } ; std :: fs :: copy (dir . join ("target/release/rustfmt") , & binary_path) ? ; return Ok (RustfmtRunner { ld_library_path : ld_lib_path , binary_path , }) ; }
};
}
