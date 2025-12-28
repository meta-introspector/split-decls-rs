macro_rules! deps {
    () => {
        ElfComdatIterator!();
        Endianness!();
        Endian!();
        FileHeader64!();
    };
}

macro_rules! ElfComdatIterator64 {
    () => {
        deps!();
        # [doc = " An iterator for the COMDAT section groups in an [`ElfFile64`](super::ElfFile64)."] pub type ElfComdatIterator64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfComdatIterator < 'data , 'file , elf :: FileHeader64 < Endian > , R > ;
    };
}

ElfComdatIterator64!();