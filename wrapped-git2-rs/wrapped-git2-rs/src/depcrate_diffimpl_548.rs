// Generated macro for impl_548 (impl)
macro_rules! Depcrate_diffimpl_548 {
() => {
// Module: crate::diff
// Provides: {"impl_548"}
// Dependencies: {}
impl DiffStats { # [doc = " Get the total number of files changed in a diff."] pub fn files_changed (& self) -> usize { unsafe { raw :: git_diff_stats_files_changed (& * self . raw) as usize } } # [doc = " Get the total number of insertions in a diff"] pub fn insertions (& self) -> usize { unsafe { raw :: git_diff_stats_insertions (& * self . raw) as usize } } # [doc = " Get the total number of deletions in a diff"] pub fn deletions (& self) -> usize { unsafe { raw :: git_diff_stats_deletions (& * self . raw) as usize } } # [doc = " Print diff statistics to a Buf"] pub fn to_buf (& self , format : DiffStatsFormat , width : usize) -> Result < Buf , Error > { let buf = Buf :: new () ; unsafe { try_call ! (raw :: git_diff_stats_to_buf (buf . raw () , self . raw , format . bits () , width as size_t)) ; } Ok (buf) } }
};
}
