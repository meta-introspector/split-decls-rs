macro_rules! deps {
    () => {
        ElfFile!();
        ProgramHeader!();
        ReadRef!();
        FileHeader!();
    };
}

macro_rules! ElfSegmentIterator {
    () => {
        deps!();
        # [doc = " An iterator for the segments in an [`ElfFile`]."] # [derive (Debug)] pub struct ElfSegmentIterator < 'data , 'file , Elf , R = & 'data [u8] > where Elf : FileHeader , R : ReadRef < 'data > , { pub (super) file : & 'file ElfFile < 'data , Elf , R > , pub (super) iter : slice :: Iter < 'data , Elf :: ProgramHeader > , }
    };
}

ElfSegmentIterator!();