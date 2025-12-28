macro_rules! deps {
    () => {
        Endian!();
        Endianness!();
        ElfComdatSectionIterator!();
        FileHeader32!();
    };
}

macro_rules! ElfComdatSectionIterator32 {
    () => {
        deps!();
        # [doc = " An iterator for the sections in a COMDAT section group in an [`ElfFile32`](super::ElfFile32)."] pub type ElfComdatSectionIterator32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfComdatSectionIterator < 'data , 'file , elf :: FileHeader32 < Endian > , R > ;
    };
}

ElfComdatSectionIterator32!()