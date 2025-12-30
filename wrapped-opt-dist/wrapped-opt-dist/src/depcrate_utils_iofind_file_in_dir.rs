// Generated macro for find_file_in_dir (function)
macro_rules! Depcrate_utils_iofind_file_in_dir {
() => {
// Module: crate::utils::io
// Provides: {"find_file_in_dir"}
// Dependencies: {}
# [doc = " Finds a single file in the specified `directory` with the given `prefix` and `suffix`."] pub fn find_file_in_dir (directory : & Utf8Path , prefix : & str , suffix : & str ,) -> anyhow :: Result < Utf8PathBuf > { let files = glob :: glob (& format ! ("{directory}/{prefix}*{suffix}")) ? . collect :: < Result < Vec < _ > , _ > > () ? ; match files . len () { 0 => Err (anyhow :: anyhow ! ("No file with prefix {prefix} found in {directory}")) , 1 => Ok (Utf8PathBuf :: from_path_buf (files [0] . clone ()) . unwrap ()) , _ => Err (anyhow :: anyhow ! ("More than one file with prefix {prefix} found in {directory}: {:?}" , files)) , } }
};
}
