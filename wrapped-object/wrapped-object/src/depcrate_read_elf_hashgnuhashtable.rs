// Generated macro for GnuHashTable (struct)
macro_rules! Depcrate_read_elf_hashGnuHashTable {
() => {
// Module: crate::read::elf::hash
// Provides: {"GnuHashTable"}
// Dependencies: {}
# [doc = " A GNU symbol hash table in an ELF file."] # [doc = ""] # [doc = " Returned by [`SectionHeader::gnu_hash`](super::SectionHeader::gnu_hash)."] # [derive (Debug)] pub struct GnuHashTable < 'data , Elf : FileHeader > { symbol_base : u32 , bloom_shift : u32 , bloom_filters : & 'data [u8] , buckets : & 'data [U32 < Elf :: Endian >] , values : & 'data [U32 < Elf :: Endian >] , }
};
}
