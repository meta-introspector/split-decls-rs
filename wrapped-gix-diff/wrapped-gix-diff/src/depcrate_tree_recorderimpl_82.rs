// Generated macro for impl_82 (impl)
macro_rules! Depcrate_tree_recorderimpl_82 {
() => {
// Module: crate::tree::recorder
// Provides: {"impl_82"}
// Dependencies: {}
# [doc = " Access"] impl Recorder { # [doc = " Obtain a copy of the currently tracked, full path of the entry."] pub fn path_clone (& self) -> BString { self . path . clone () } # [doc = " Return the currently set path."] pub fn path (& self) -> & BStr { self . path . as_ref () } }
};
}
