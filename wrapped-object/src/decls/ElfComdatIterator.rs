macro_rules! deps {
    () => {
        FileHeader!();
        ReadRef!();
        ElfFile!();
        SectionHeader!();
    };
}

macro_rules! ElfComdatIterator {
    () => {
        deps!();
        # [doc = " An iterator for the COMDAT section groups in an [`ElfFile`]."] # [derive (Debug)] pub struct ElfComdatIterator < 'data , 'file , Elf , R = & 'data [u8] > where Elf : FileHeader , R : ReadRef < 'data > , { file : & 'file ElfFile < 'data , Elf , R > , iter : iter :: Enumerate < slice :: Iter < 'data , Elf :: SectionHeader > > , }
    };
}

ElfComdatIterator!();