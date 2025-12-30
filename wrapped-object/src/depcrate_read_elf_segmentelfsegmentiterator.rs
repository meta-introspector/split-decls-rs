// Generated macro for ElfSegmentIterator (struct)
macro_rules! Depcrate_read_elf_segmentElfSegmentIterator {
() => {
// Module: crate::read::elf::segment
// Provides: {"ElfSegmentIterator"}
// Dependencies: {}
# [doc = " An iterator for the segments in an [`ElfFile`]."] # [derive (Debug)] pub struct ElfSegmentIterator < 'data , 'file , Elf , R = & 'data [u8] > where Elf : FileHeader , R : ReadRef < 'data > , { pub (super) file : & 'file ElfFile < 'data , Elf , R > , pub (super) iter : slice :: Iter < 'data , Elf :: ProgramHeader > , }
};
}
