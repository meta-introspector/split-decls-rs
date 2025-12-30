// Generated macro for VerneedIterator (struct)
macro_rules! Depcrate_read_elf_versionVerneedIterator {
() => {
// Module: crate::read::elf::version
// Provides: {"VerneedIterator"}
// Dependencies: {}
# [doc = " An iterator for the entries in an ELF [`elf::SHT_GNU_VERNEED`] section."] # [derive (Debug , Clone)] pub struct VerneedIterator < 'data , Elf : FileHeader > { endian : Elf :: Endian , data : Bytes < 'data > , }
};
}
