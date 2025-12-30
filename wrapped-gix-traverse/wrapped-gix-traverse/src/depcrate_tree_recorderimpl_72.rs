// Generated macro for impl_72 (impl)
macro_rules! Depcrate_tree_recorderimpl_72 {
() => {
// Module: crate::tree::recorder
// Provides: {"impl_72"}
// Dependencies: {}
# [doc = " Access"] impl Recorder { # [doc = " Obtain a copy of the currently tracked, full path of the entry."] pub fn path_clone (& self) -> BString { self . path . clone () } # [doc = " Return the currently set path."] pub fn path (& self) -> & BStr { self . path . as_ref () } }
};
}
