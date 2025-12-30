// Generated macro for get_modified_rs_files (function)
macro_rules! Depcrate_core_build_steps_formatget_modified_rs_files {
() => {
// Module: crate::core::build_steps::format
// Provides: {"get_modified_rs_files"}
// Dependencies: {}
# [doc = " Returns the Rust files modified between the last merge commit and what is now on the disk."] # [doc = " Does not include removed files."] # [doc = ""] # [doc = " Returns `None` if all files should be formatted."] fn get_modified_rs_files (build : & Builder < '_ >) -> Result < Option < Vec < String > > , String > { assert ! (! build . config . is_running_on_ci) ; if ! verify_rustfmt_version (build) { return Ok (None) ; } get_git_modified_files (& build . config . git_config () , Some (& build . config . src) , & ["rs"]) . map (Some) }
};
}
