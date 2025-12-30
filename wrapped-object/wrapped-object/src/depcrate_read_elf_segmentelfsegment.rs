// Generated macro for ElfSegment (struct)
macro_rules! Depcrate_read_elf_segmentElfSegment {
() => {
// Module: crate::read::elf::segment
// Provides: {"ElfSegment"}
// Dependencies: {}
# [doc = " A segment in an [`ElfFile`]."] # [doc = ""] # [doc = " Most functionality is provided by the [`ObjectSegment`] trait implementation."] # [derive (Debug)] pub struct ElfSegment < 'data , 'file , Elf , R = & 'data [u8] > where Elf : FileHeader , R : ReadRef < 'data > , { pub (super) file : & 'file ElfFile < 'data , Elf , R > , pub (super) segment : & 'data Elf :: ProgramHeader , }
};
}
