// Generated macro for split_debuginfo (function)
macro_rules! Depcrate_utils_helperssplit_debuginfo {
() => {
// Module: crate::utils::helpers
// Provides: {"split_debuginfo"}
// Dependencies: {}
# [doc = " Returns the path to the split debug info for the specified file if it exists."] pub fn split_debuginfo (name : impl Into < PathBuf >) -> Option < PathBuf > { let path = name . into () ; let pdb = path . with_extension ("pdb") ; if pdb . exists () { return Some (pdb) ; } let file_name = pdb . file_name () ? . to_str () ? . replace ("-" , "_") ; let pdb : PathBuf = [path . parent () ? , Path :: new (& file_name)] . into_iter () . collect () ; pdb . exists () . then_some (pdb) }
};
}
