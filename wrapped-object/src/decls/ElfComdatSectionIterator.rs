macro_rules! deps {
    () => {
        ElfFile!();
        ReadRef!();
        FileHeader!();
        U32Bytes!();
        Endian!();
    };
}

macro_rules! ElfComdatSectionIterator {
    () => {
        deps!();
        # [doc = " An iterator for the sections in a COMDAT section group in an [`ElfFile`]."] # [derive (Debug)] pub struct ElfComdatSectionIterator < 'data , 'file , Elf , R = & 'data [u8] > where Elf : FileHeader , R : ReadRef < 'data > , { file : & 'file ElfFile < 'data , Elf , R > , sections : slice :: Iter < 'data , U32Bytes < Elf :: Endian > > , }
    };
}

ElfComdatSectionIterator!();