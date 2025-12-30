// Generated macro for PeSectionIterator (struct)
macro_rules! Depcrate_read_pe_sectionPeSectionIterator {
() => {
// Module: crate::read::pe::section
// Provides: {"PeSectionIterator"}
// Dependencies: {}
# [doc = " An iterator for the sections in a [`PeFile`]."] # [derive (Debug)] pub struct PeSectionIterator < 'data , 'file , Pe , R = & 'data [u8] > where Pe : ImageNtHeaders , R : ReadRef < 'data > , { pub (super) file : & 'file PeFile < 'data , Pe , R > , pub (super) iter : iter :: Enumerate < slice :: Iter < 'data , pe :: ImageSectionHeader > > , }
};
}
