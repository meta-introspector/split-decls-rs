// Generated macro for impl_595 (impl)
macro_rules! Depcrate_output_file_nameimpl_595 {
() => {
// Module: crate::output::file_name
// Provides: {"impl_595"}
// Dependencies: {}
impl < 'a , 'dir , C > FileName < 'a , 'dir , C > { # [doc = " Sets the flag on this file name to display link targets with an"] # [doc = " arrow followed by their path."] pub fn with_link_paths (mut self) -> Self { if ! self . file . deref_links { self . link_style = LinkStyle :: FullLinkPaths ; } self } # [doc = " Sets the flag on this file name to display mounted filesystem"] # [doc = "details."] pub fn with_mount_details (mut self , enable : bool) -> Self { self . mount_style = if enable { MountStyle :: MountInfo } else { MountStyle :: JustDirectoryNames } ; self } }
};
}
