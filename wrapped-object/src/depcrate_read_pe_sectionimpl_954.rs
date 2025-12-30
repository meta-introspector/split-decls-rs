// Generated macro for impl_954 (impl)
macro_rules! Depcrate_read_pe_sectionimpl_954 {
() => {
// Module: crate::read::pe::section
// Provides: {"impl_954"}
// Dependencies: {}
impl < 'data , 'file , Pe , R > PeSegment < 'data , 'file , Pe , R > where Pe : ImageNtHeaders , R : ReadRef < 'data > , { # [doc = " Get the PE file containing this segment."] pub fn pe_file (& self) -> & 'file PeFile < 'data , Pe , R > { self . file } # [doc = " Get the raw PE section header."] pub fn pe_section (& self) -> & 'data pe :: ImageSectionHeader { self . section } }
};
}
