macro_rules! deps {
    () => {
        FileHeader!();
        ElfSymbolTable!();
        ReadRef!();
    };
}

macro_rules! impl_327 {
    () => {
        deps!();
        impl < 'data , 'file , Elf : FileHeader , R : ReadRef < 'data > > read :: private :: Sealed for ElfSymbolTable < 'data , 'file , Elf , R > { }
    };
}

impl_327!()