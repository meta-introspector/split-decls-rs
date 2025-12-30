// Generated macro for ElfRelocationIterator (enum)
macro_rules! Depcrate_read_elf_relocationElfRelocationIterator {
() => {
// Module: crate::read::elf::relocation
// Provides: {"ElfRelocationIterator"}
// Dependencies: {}
pub (super) enum ElfRelocationIterator < 'data , Elf : FileHeader > { Rel (slice :: Iter < 'data , Elf :: Rel > , Elf :: Endian) , Rela (slice :: Iter < 'data , Elf :: Rela > , Elf :: Endian , bool) , Crel (CrelIterator < 'data >) , }
};
}
