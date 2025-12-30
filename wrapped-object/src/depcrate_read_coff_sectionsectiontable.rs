// Generated macro for SectionTable (struct)
macro_rules! Depcrate_read_coff_sectionSectionTable {
() => {
// Module: crate::read::coff::section
// Provides: {"SectionTable"}
// Dependencies: {}
# [doc = " The table of section headers in a COFF or PE file."] # [doc = ""] # [doc = " Returned by [`CoffHeader::sections`] and"] # [doc = " [`ImageNtHeaders::sections`](crate::read::pe::ImageNtHeaders::sections)."] # [derive (Debug , Default , Clone , Copy)] pub struct SectionTable < 'data > { sections : & 'data [pe :: ImageSectionHeader] , }
};
}
