macro_rules! deps {
    () => {
        Endianness!();
        FileHeader32!();
        Endian!();
        ElfSectionIterator!();
    };
}

macro_rules! ElfSectionIterator32 {
    () => {
        deps!();
        # [doc = " An iterator for the sections in an [`ElfFile32`](super::ElfFile32)."] pub type ElfSectionIterator32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfSectionIterator < 'data , 'file , elf :: FileHeader32 < Endian > , R > ;
    };
}

ElfSectionIterator32!()