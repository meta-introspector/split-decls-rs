// Generated macro for VernauxIterator (struct)
macro_rules! Depcrate_read_elf_versionVernauxIterator {
() => {
// Module: crate::read::elf::version
// Provides: {"VernauxIterator"}
// Dependencies: {}
# [doc = " An iterator for the auxiliary records for an entry in an ELF [`elf::SHT_GNU_VERNEED`] section."] # [derive (Debug , Clone)] pub struct VernauxIterator < 'data , Elf : FileHeader > { endian : Elf :: Endian , data : Bytes < 'data > , count : u16 , }
};
}
