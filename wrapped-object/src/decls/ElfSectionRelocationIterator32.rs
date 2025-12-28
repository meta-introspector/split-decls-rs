macro_rules! deps {
    () => {
        ElfSectionRelocationIterator!();
        Endian!();
        FileHeader32!();
        Endianness!();
    };
}

macro_rules! ElfSectionRelocationIterator32 {
    () => {
        deps!();
        # [doc = " An iterator for the relocations for an [`ElfSection32`](super::ElfSection32)."] pub type ElfSectionRelocationIterator32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfSectionRelocationIterator < 'data , 'file , elf :: FileHeader32 < Endian > , R > ;
    };
}

ElfSectionRelocationIterator32!()