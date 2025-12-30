// Generated macro for CoffSection (struct)
macro_rules! Depcrate_read_coff_sectionCoffSection {
() => {
// Module: crate::read::coff::section
// Provides: {"CoffSection"}
// Dependencies: {}
# [doc = " A section in a [`CoffFile`]."] # [doc = ""] # [doc = " Most functionality is provided by the [`ObjectSection`] trait implementation."] # [derive (Debug)] pub struct CoffSection < 'data , 'file , R : ReadRef < 'data > = & 'data [u8] , Coff : CoffHeader = pe :: ImageFileHeader , > { pub (super) file : & 'file CoffFile < 'data , R , Coff > , pub (super) index : SectionIndex , pub (super) section : & 'data pe :: ImageSectionHeader , }
};
}
