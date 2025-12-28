macro_rules! deps {
    () => {
        ReadRef!();
        ElfSymbolTable!();
        FileHeader!();
    };
}

macro_rules! impl_327 {
    () => {
        deps!();
        impl < 'data , 'file , Elf : FileHeader , R : ReadRef < 'data > > read :: private :: Sealed for ElfSymbolTable < 'data , 'file , Elf , R > { }
    };
}

impl_327!();