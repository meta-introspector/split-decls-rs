macro_rules! deps {
    () => {
        Endian!();
        FileHeader64!();
        ElfDynamicRelocationIterator!();
        Endianness!();
    };
}

macro_rules! ElfDynamicRelocationIterator64 {
    () => {
        deps!();
        # [doc = " An iterator for the dynamic relocations in an [`ElfFile64`](super::ElfFile64)."] pub type ElfDynamicRelocationIterator64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfDynamicRelocationIterator < 'data , 'file , elf :: FileHeader64 < Endian > , R > ;
    };
}

ElfDynamicRelocationIterator64!()