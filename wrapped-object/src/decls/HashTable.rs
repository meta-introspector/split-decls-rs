macro_rules! deps {
    () => {
        Endian!();
        FileHeader!();
        U32!();
    };
}

macro_rules! HashTable {
    () => {
        deps!();
        # [doc = " A SysV symbol hash table in an ELF file."] # [doc = ""] # [doc = " Returned by [`SectionHeader::hash`](super::SectionHeader::hash)."] # [derive (Debug)] pub struct HashTable < 'data , Elf : FileHeader > { buckets : & 'data [U32 < Elf :: Endian >] , chains : & 'data [U32 < Elf :: Endian >] , }
    };
}

HashTable!()