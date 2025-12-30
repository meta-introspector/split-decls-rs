// Generated macro for VerdefIterator (struct)
macro_rules! Depcrate_read_elf_versionVerdefIterator {
() => {
// Module: crate::read::elf::version
// Provides: {"VerdefIterator"}
// Dependencies: {}
# [doc = " An iterator for the entries in an ELF [`elf::SHT_GNU_VERDEF`] section."] # [derive (Debug , Clone)] pub struct VerdefIterator < 'data , Elf : FileHeader > { endian : Elf :: Endian , data : Bytes < 'data > , }
};
}
