// Generated macro for impl_420 (impl)
macro_rules! Depcrate_read_elf_segmentimpl_420 {
() => {
// Module: crate::read::elf::segment
// Provides: {"impl_420"}
// Dependencies: {}
impl < 'data , 'file , Elf : FileHeader , R : ReadRef < 'data > > ElfSegment < 'data , 'file , Elf , R > { # [doc = " Get the ELF file containing this segment."] pub fn elf_file (& self) -> & 'file ElfFile < 'data , Elf , R > { self . file } # [doc = " Get the raw ELF program header for the segment."] pub fn elf_program_header (& self) -> & 'data Elf :: ProgramHeader { self . segment } fn bytes (& self) -> read :: Result < & 'data [u8] > { self . segment . data (self . file . endian , self . file . data) . read_error ("Invalid ELF segment size or offset") } }
};
}
