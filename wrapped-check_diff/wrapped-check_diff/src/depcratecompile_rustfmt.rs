// Generated macro for compile_rustfmt (function)
macro_rules! Depcratecompile_rustfmt {
() => {
// Module: crate
// Provides: {"compile_rustfmt"}
// Dependencies: {}
pub fn compile_rustfmt (dest : & Path , remote_repo_url : String , feature_branch : String , commit_hash : Option < String > ,) -> Result < CheckDiffRunners < RustfmtRunner , RustfmtRunner > , CheckDiffError > { const RUSTFMT_REPO : & str = "https://github.com/rust-lang/rustfmt.git" ; clone_git_repo (RUSTFMT_REPO , dest) ? ; change_directory_to_path (dest) ? ; git_remote_add (remote_repo_url . as_str ()) ? ; git_fetch (feature_branch . as_str ()) ? ; let cargo_version = get_cargo_version () ? ; info ! ("Compiling with {}" , cargo_version) ; let src_runner = build_rustfmt_from_src (dest . join ("src_rustfmt") , dest) ? ; let should_detach = commit_hash . is_some () ; git_switch (commit_hash . unwrap_or (feature_branch) . as_str () , should_detach ,) ? ; let feature_runner = build_rustfmt_from_src (dest . join ("feature_rustfmt") , dest) ? ; info ! ("RUSFMT_BIN {}" , src_runner . get_binary_version () ?) ; info ! ("Runtime dependencies for (src) rustfmt -- LD_LIBRARY_PATH: {}" , src_runner . ld_library_path) ; info ! ("FEATURE_BIN {}" , feature_runner . get_binary_version () ?) ; info ! ("Runtime dependencies for (feature) rustfmt -- LD_LIBRARY_PATH: {}" , feature_runner . ld_library_path) ; return Ok (CheckDiffRunners { src_runner , feature_runner , }) ; }
};
}
