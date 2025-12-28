macro_rules! deps {
    () => {
        FileHeader!();
        Result!();
        ElfDynamicRelocationIterator!();
        ReadRef!();
    };
}

macro_rules! impl_354 {
    () => {
        deps!();
        impl < 'data , 'file , Elf , R > fmt :: Debug for ElfDynamicRelocationIterator < 'data , 'file , Elf , R > where Elf : FileHeader , R : ReadRef < 'data > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ElfDynamicRelocationIterator") . finish () } }
    };
}

impl_354!();