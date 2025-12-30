// Generated macro for count_files (function)
macro_rules! Depcrate_utils_iocount_files {
() => {
// Module: crate::utils::io
// Provides: {"count_files"}
// Dependencies: {}
# [doc = " Counts all children of a directory (non-recursively)."] pub fn count_files (dir : & Utf8Path) -> anyhow :: Result < u64 > { Ok (std :: fs :: read_dir (dir) ? . count () as u64) }
};
}
