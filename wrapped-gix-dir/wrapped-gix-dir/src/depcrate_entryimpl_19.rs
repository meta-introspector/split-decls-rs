// Generated macro for impl_19 (impl)
macro_rules! Depcrate_entryimpl_19 {
() => {
// Module: crate::entry
// Provides: {"impl_19"}
// Dependencies: {}
impl Kind { pub (super) fn is_recursable_dir (& self) -> bool { matches ! (self , Kind :: Directory) } # [doc = " Return `true` if this is a directory on disk. Note that this is true for repositories as well."] pub fn is_dir (& self) -> bool { matches ! (self , Kind :: Directory | Kind :: Repository) } }
};
}
