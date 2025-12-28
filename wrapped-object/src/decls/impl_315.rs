macro_rules! deps {
    () => {
        ElfSection!();
        ReadRef!();
        FileHeader!();
    };
}

macro_rules! impl_315 {
    () => {
        deps!();
        impl < 'data , 'file , Elf , R > read :: private :: Sealed for ElfSection < 'data , 'file , Elf , R > where Elf : FileHeader , R : ReadRef < 'data > , { }
    };
}

impl_315!();