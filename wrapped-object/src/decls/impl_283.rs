macro_rules! deps {
    () => {
        FileHeader!();
        ElfFile!();
        ReadRef!();
    };
}

macro_rules! impl_283 {
    () => {
        deps!();
        impl < 'data , Elf , R > read :: private :: Sealed for ElfFile < 'data , Elf , R > where Elf : FileHeader , R : ReadRef < 'data > , { }
    };
}

impl_283!();