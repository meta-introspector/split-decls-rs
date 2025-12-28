macro_rules! deps {
    () => {
        CrelIterator!();
        FileHeader!();
        Crel!();
        Rela!();
        Rel!();
        Endian!();
    };
}

macro_rules! ElfRelocationIterator {
    () => {
        deps!();
        pub (super) enum ElfRelocationIterator < 'data , Elf : FileHeader > { Rel (slice :: Iter < 'data , Elf :: Rel > , Elf :: Endian) , Rela (slice :: Iter < 'data , Elf :: Rela > , Elf :: Endian , bool) , Crel (CrelIterator < 'data >) , }
    };
}

ElfRelocationIterator!();