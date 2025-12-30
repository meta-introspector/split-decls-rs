// Generated macro for SectionTable (struct)
macro_rules! Depcrate_read_xcoff_sectionSectionTable {
() => {
// Module: crate::read::xcoff::section
// Provides: {"SectionTable"}
// Dependencies: {}
# [doc = " The table of section headers in an XCOFF file."] # [doc = ""] # [doc = " Returned by [`FileHeader::sections`]."] # [derive (Debug , Clone , Copy)] pub struct SectionTable < 'data , Xcoff : FileHeader > { sections : & 'data [Xcoff :: SectionHeader] , }
};
}
