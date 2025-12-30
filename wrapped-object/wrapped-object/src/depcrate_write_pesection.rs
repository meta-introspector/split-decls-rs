// Generated macro for Section (struct)
macro_rules! Depcrate_write_peSection {
() => {
// Module: crate::write::pe
// Provides: {"Section"}
// Dependencies: {}
# [doc = " Information required for writing [`pe::ImageSectionHeader`]."] # [allow (missing_docs)] # [derive (Debug , Clone)] pub struct Section { pub name : [u8 ; pe :: IMAGE_SIZEOF_SHORT_NAME] , pub characteristics : u32 , pub range : SectionRange , }
};
}
