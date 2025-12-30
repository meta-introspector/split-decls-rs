// Generated macro for VerdauxIterator (struct)
macro_rules! Depcrate_read_elf_versionVerdauxIterator {
() => {
// Module: crate::read::elf::version
// Provides: {"VerdauxIterator"}
// Dependencies: {}
# [doc = " An iterator for the auxiliary records for an entry in an ELF [`elf::SHT_GNU_VERDEF`] section."] # [derive (Debug , Clone)] pub struct VerdauxIterator < 'data , Elf : FileHeader > { endian : Elf :: Endian , data : Bytes < 'data > , count : u16 , }
};
}
