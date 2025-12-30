// Generated macro for HashTable (struct)
macro_rules! Depcrate_read_elf_hashHashTable {
() => {
// Module: crate::read::elf::hash
// Provides: {"HashTable"}
// Dependencies: {}
# [doc = " A SysV symbol hash table in an ELF file."] # [doc = ""] # [doc = " Returned by [`SectionHeader::hash`](super::SectionHeader::hash)."] # [derive (Debug)] pub struct HashTable < 'data , Elf : FileHeader > { buckets : & 'data [U32 < Elf :: Endian >] , chains : & 'data [U32 < Elf :: Endian >] , }
};
}
