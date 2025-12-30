// Generated macro for impl_96 (impl)
macro_rules! Depcrate_blob_platform_resourceimpl_96 {
() => {
// Module: crate::blob::platform::resource
// Provides: {"impl_96"}
// Dependencies: {}
impl < 'a > Data < 'a > { # [doc = " Return ourselves as slice of bytes if this instance stores data."] # [doc = " Note that missing data is interpreted as empty slice, to facilitate additions and deletions."] pub fn as_slice (& self) -> Option < & 'a [u8] > { match self { Data :: Buffer (d) => Some (d) , Data :: Missing => Some (& []) , Data :: TooLarge { .. } => None , } } }
};
}
