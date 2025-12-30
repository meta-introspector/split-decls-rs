// Generated macro for SectionHeader (struct)
macro_rules! Depcrate_write_coff_writerSectionHeader {
() => {
// Module: crate::write::coff::writer
// Provides: {"SectionHeader"}
// Dependencies: {}
# [doc = " Native endian version of [`pe::ImageSectionHeader`]."] # [allow (missing_docs)] # [derive (Debug , Default , Clone)] pub struct SectionHeader { pub name : Name , pub size_of_raw_data : u32 , pub pointer_to_raw_data : u32 , pub pointer_to_relocations : u32 , pub pointer_to_linenumbers : u32 , # [doc = " This will automatically be clamped if there are more than 0xffff."] pub number_of_relocations : u32 , pub number_of_linenumbers : u16 , pub characteristics : u32 , }
};
}
