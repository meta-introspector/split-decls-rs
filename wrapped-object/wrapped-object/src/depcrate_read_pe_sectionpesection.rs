// Generated macro for PeSection (struct)
macro_rules! Depcrate_read_pe_sectionPeSection {
() => {
// Module: crate::read::pe::section
// Provides: {"PeSection"}
// Dependencies: {}
# [doc = " A section in a [`PeFile`]."] # [doc = ""] # [doc = " Most functionality is provided by the [`ObjectSection`] trait implementation."] # [derive (Debug)] pub struct PeSection < 'data , 'file , Pe , R = & 'data [u8] > where Pe : ImageNtHeaders , R : ReadRef < 'data > , { pub (super) file : & 'file PeFile < 'data , Pe , R > , pub (super) index : SectionIndex , pub (super) section : & 'data pe :: ImageSectionHeader , }
};
}
