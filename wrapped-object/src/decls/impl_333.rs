macro_rules! deps {
    () => {
        FileHeader!();
        ReadRef!();
        ElfSymbolIterator!();
        Result!();
    };
}

macro_rules! impl_333 {
    () => {
        deps!();
        impl < 'data , 'file , Elf : FileHeader , R : ReadRef < 'data > > fmt :: Debug for ElfSymbolIterator < 'data , 'file , Elf , R > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ElfSymbolIterator") . finish () } }
    };
}

impl_333!();