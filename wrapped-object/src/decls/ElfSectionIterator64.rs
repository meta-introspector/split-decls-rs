macro_rules! deps {
    () => {
        Endianness!();
        ElfSectionIterator!();
        FileHeader64!();
        Endian!();
    };
}

macro_rules! ElfSectionIterator64 {
    () => {
        deps!();
        # [doc = " An iterator for the sections in an [`ElfFile64`](super::ElfFile64)."] pub type ElfSectionIterator64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfSectionIterator < 'data , 'file , elf :: FileHeader64 < Endian > , R > ;
    };
}

ElfSectionIterator64!()