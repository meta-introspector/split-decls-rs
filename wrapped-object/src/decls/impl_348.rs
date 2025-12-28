macro_rules! deps {
    () => {
        Rel!();
        Crel!();
        FileHeader!();
        ElfRelocationIterator!();
        Rela!();
    };
}

macro_rules! impl_348 {
    () => {
        deps!();
        impl < 'data , Elf : FileHeader > ElfRelocationIterator < 'data , Elf > { fn is_rel (& self) -> bool { match self { ElfRelocationIterator :: Rel (..) => true , ElfRelocationIterator :: Rela (..) => false , ElfRelocationIterator :: Crel (i) => ! i . is_rela () , } } }
    };
}

impl_348!();