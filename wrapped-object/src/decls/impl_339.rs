macro_rules! deps {
    () => {
        ElfSymbol!();
        ReadRef!();
        FileHeader!();
    };
}

macro_rules! impl_339 {
    () => {
        deps!();
        impl < 'data , 'file , Elf : FileHeader , R : ReadRef < 'data > > read :: private :: Sealed for ElfSymbol < 'data , 'file , Elf , R > { }
    };
}

impl_339!();