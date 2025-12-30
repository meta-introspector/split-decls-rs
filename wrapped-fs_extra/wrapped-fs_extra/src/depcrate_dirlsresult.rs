// Generated macro for LsResult (struct)
macro_rules! Depcrate_dirLsResult {
() => {
// Module: crate::dir
// Provides: {"LsResult"}
// Dependencies: {}
# [doc = " Result returned by the `ls` function."] pub struct LsResult { # [doc = " Base folder target path"] pub base : HashMap < DirEntryAttr , DirEntryValue > , # [doc = " Collection directory entry with information."] pub items : Vec < HashMap < DirEntryAttr , DirEntryValue > > , }
};
}
