// Generated macro for impl_416 (impl)
macro_rules! Depcrate_read_elf_segmentimpl_416 {
() => {
// Module: crate::read::elf::segment
// Provides: {"impl_416"}
// Dependencies: {}
impl < 'data , 'file , Elf , R > Iterator for ElfSegmentIterator < 'data , 'file , Elf , R > where Elf : FileHeader , R : ReadRef < 'data > , { type Item = ElfSegment < 'data , 'file , Elf , R > ; fn next (& mut self) -> Option < Self :: Item > { for segment in self . iter . by_ref () { if segment . p_type (self . file . endian) == elf :: PT_LOAD { return Some (ElfSegment { file : self . file , segment , }) ; } } None } }
};
}
