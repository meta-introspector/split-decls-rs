macro_rules! deps {
    () => {
        ElfComdatIterator!();
        Endianness!();
        FileHeader32!();
        Endian!();
    };
}

macro_rules! ElfComdatIterator32 {
    () => {
        deps!();
        # [doc = " An iterator for the COMDAT section groups in an [`ElfFile32`](super::ElfFile32)."] pub type ElfComdatIterator32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfComdatIterator < 'data , 'file , elf :: FileHeader32 < Endian > , R > ;
    };
}

ElfComdatIterator32!()