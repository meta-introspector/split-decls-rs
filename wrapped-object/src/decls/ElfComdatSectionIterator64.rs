macro_rules! deps {
    () => {
        ElfComdatSectionIterator!();
        Endian!();
        Endianness!();
        FileHeader64!();
    };
}

macro_rules! ElfComdatSectionIterator64 {
    () => {
        deps!();
        # [doc = " An iterator for the sections in a COMDAT section group in an [`ElfFile64`](super::ElfFile64)."] pub type ElfComdatSectionIterator64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfComdatSectionIterator < 'data , 'file , elf :: FileHeader64 < Endian > , R > ;
    };
}

ElfComdatSectionIterator64!();