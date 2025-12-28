macro_rules! deps {
    () => {
        U32!();
        Endian!();
        FileHeader!();
    };
}

macro_rules! GnuHashTable {
    () => {
        deps!();
        # [doc = " A GNU symbol hash table in an ELF file."] # [doc = ""] # [doc = " Returned by [`SectionHeader::gnu_hash`](super::SectionHeader::gnu_hash)."] # [derive (Debug)] pub struct GnuHashTable < 'data , Elf : FileHeader > { symbol_base : u32 , bloom_shift : u32 , bloom_filters : & 'data [u8] , buckets : & 'data [U32 < Elf :: Endian >] , values : & 'data [U32 < Elf :: Endian >] , }
    };
}

GnuHashTable!()