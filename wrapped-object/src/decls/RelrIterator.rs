macro_rules! deps {
    () => {
        FileHeader!();
        Relr!();
        Endian!();
    };
}

macro_rules! RelrIterator {
    () => {
        deps!();
        # [doc = " An iterator over the relative relocations in an ELF `SHT_RELR` section."] # [doc = ""] # [doc = " Returned by [`SectionHeader::relr`](super::SectionHeader::relr)."] # [derive (Debug)] pub struct RelrIterator < 'data , Elf : FileHeader > { offset : Elf :: Word , bits : Elf :: Word , count : u8 , iter : slice :: Iter < 'data , Elf :: Relr > , endian : Elf :: Endian , }
    };
}

RelrIterator!();