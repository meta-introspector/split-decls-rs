macro_rules! deps {
    () => {
        ElfSectionRelocationIterator!();
        ReadRef!();
        Result!();
        FileHeader!();
    };
}

macro_rules! impl_359 {
    () => {
        deps!();
        impl < 'data , 'file , Elf , R > fmt :: Debug for ElfSectionRelocationIterator < 'data , 'file , Elf , R > where Elf : FileHeader , R : ReadRef < 'data > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ElfSectionRelocationIterator") . finish () } }
    };
}

impl_359!();