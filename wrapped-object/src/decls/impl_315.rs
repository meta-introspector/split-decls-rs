macro_rules! deps {
    () => {
        ReadRef!();
        FileHeader!();
        ElfSection!();
    };
}

macro_rules! impl_315 {
    () => {
        deps!();
        impl < 'data , 'file , Elf , R > read :: private :: Sealed for ElfSection < 'data , 'file , Elf , R > where Elf : FileHeader , R : ReadRef < 'data > , { }
    };
}

impl_315!()