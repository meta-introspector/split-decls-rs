macro_rules! deps {
    () => {
        Endian!();
        ElfSectionRelocationIterator!();
        Endianness!();
        FileHeader64!();
    };
}

macro_rules! ElfSectionRelocationIterator64 {
    () => {
        deps!();
        # [doc = " An iterator for the relocations for an [`ElfSection64`](super::ElfSection64)."] pub type ElfSectionRelocationIterator64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfSectionRelocationIterator < 'data , 'file , elf :: FileHeader64 < Endian > , R > ;
    };
}

ElfSectionRelocationIterator64!()