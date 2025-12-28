macro_rules! deps {
    () => {
        Endian!();
        Endianness!();
        ElfDynamicRelocationIterator!();
        FileHeader32!();
    };
}

macro_rules! ElfDynamicRelocationIterator32 {
    () => {
        deps!();
        # [doc = " An iterator for the dynamic relocations in an [`ElfFile32`](super::ElfFile32)."] pub type ElfDynamicRelocationIterator32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfDynamicRelocationIterator < 'data , 'file , elf :: FileHeader32 < Endian > , R > ;
    };
}

ElfDynamicRelocationIterator32!();