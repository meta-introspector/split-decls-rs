// Generated macro for PeSegment (struct)
macro_rules! Depcrate_read_pe_sectionPeSegment {
() => {
// Module: crate::read::pe::section
// Provides: {"PeSegment"}
// Dependencies: {}
# [doc = " A loadable section in a [`PeFile`]."] # [doc = ""] # [doc = " Most functionality is provided by the [`ObjectSegment`] trait implementation."] # [derive (Debug)] pub struct PeSegment < 'data , 'file , Pe , R = & 'data [u8] > where Pe : ImageNtHeaders , R : ReadRef < 'data > , { file : & 'file PeFile < 'data , Pe , R > , section : & 'data pe :: ImageSectionHeader , }
};
}
