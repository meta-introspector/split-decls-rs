// Generated macro for DirEntryInner (enum)
macro_rules! Depcrate_walkDirEntryInner {
() => {
// Module: crate::walk
// Provides: {"DirEntryInner"}
// Dependencies: {}
# [doc = " DirEntryInner is the implementation of DirEntry."] # [doc = ""] # [doc = " It specifically represents three distinct sources of directory entries:"] # [doc = ""] # [doc = " 1. From the walkdir crate."] # [doc = " 2. Special entries that represent things like stdin."] # [doc = " 3. From a path."] # [doc = ""] # [doc = " Specifically, (3) has to essentially re-create the DirEntry implementation"] # [doc = " from WalkDir."] # [derive (Clone , Debug)] enum DirEntryInner { Stdin , Walkdir (walkdir :: DirEntry) , Raw (DirEntryRaw) , }
};
}
