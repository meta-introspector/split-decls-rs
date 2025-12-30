// Generated macro for impl_62 (impl)
macro_rules! Depcrate_fs_fileimpl_62 {
() => {
// Module: crate::fs::file
// Provides: {"impl_62"}
// Dependencies: {}
impl < 'dir > FileTarget < 'dir > { # [doc = " Whether this link doesn’t lead to a file, for whatever reason. This"] # [doc = " gets used to determine how to highlight the link in grid views."] pub fn is_broken (& self) -> bool { matches ! (self , Self :: Broken (_) | Self :: Err (_)) } }
};
}
