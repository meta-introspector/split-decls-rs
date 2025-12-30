// Generated macro for macro_1384 (macro)
macro_rules! Depcratemacro_1384 {
() => {
// Module: crate
// Provides: {"macro_1384"}
// Dependencies: {}
bitflags ! { # [allow (missing_docs)] # [derive (Clone , Copy , Debug , Eq , PartialEq , PartialOrd , Ord , Hash)] pub struct StashFlags : u32 { # [allow (missing_docs)] const DEFAULT = raw :: GIT_STASH_DEFAULT as u32 ; # [doc = " All changes already added to the index are left intact in"] # [doc = " the working directory"] const KEEP_INDEX = raw :: GIT_STASH_KEEP_INDEX as u32 ; # [doc = " All untracked files are also stashed and then cleaned up"] # [doc = " from the working directory"] const INCLUDE_UNTRACKED = raw :: GIT_STASH_INCLUDE_UNTRACKED as u32 ; # [doc = " All ignored files are also stashed and then cleaned up from"] # [doc = " the working directory"] const INCLUDE_IGNORED = raw :: GIT_STASH_INCLUDE_IGNORED as u32 ; # [doc = " All changes in the index and working directory are left intact"] const KEEP_ALL = raw :: GIT_STASH_KEEP_ALL as u32 ; } }
};
}
