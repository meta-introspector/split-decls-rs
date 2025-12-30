// Generated macro for DirEntry (struct)
macro_rules! Depcrate_walkDirEntry {
() => {
// Module: crate::walk
// Provides: {"DirEntry"}
// Dependencies: {}
# [doc = " A directory entry with a possible error attached."] # [doc = ""] # [doc = " The error typically refers to a problem parsing ignore files in a"] # [doc = " particular directory."] # [derive (Clone , Debug)] pub struct DirEntry { dent : DirEntryInner , err : Option < Error > , }
};
}
