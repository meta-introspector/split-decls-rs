macro_rules! deps {
    () => {
        ReadRef!();
        ElfComdat!();
        FileHeader!();
    };
}

macro_rules! impl_390 {
    () => {
        deps!();
        impl < 'data , 'file , Elf , R > read :: private :: Sealed for ElfComdat < 'data , 'file , Elf , R > where Elf : FileHeader , R : ReadRef < 'data > , { }
    };
}

impl_390!();