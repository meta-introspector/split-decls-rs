// Generated macro for get_files_from_dir (function)
macro_rules! Depcrate_utils_ioget_files_from_dir {
() => {
// Module: crate::utils::io
// Provides: {"get_files_from_dir"}
// Dependencies: {}
# [doc = " Returns paths in the given `dir` (non-recursively), optionally with the given `suffix`."] # [doc = " The `suffix` should contain the leading dot."] pub fn get_files_from_dir (dir : & Utf8Path , suffix : Option < & str > ,) -> anyhow :: Result < Vec < Utf8PathBuf > > { let path = format ! ("{dir}/*{}" , suffix . unwrap_or ("")) ; Ok (glob :: glob (& path) ? . map (| p | p . map (| p | Utf8PathBuf :: from_path_buf (p) . unwrap ())) . collect :: < Result < Vec < _ > , _ > > () ?) }
};
}
