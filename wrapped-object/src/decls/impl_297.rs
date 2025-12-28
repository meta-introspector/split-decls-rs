macro_rules! deps {
    () => {
        FileHeader!();
        ElfSegment!();
        ReadRef!();
    };
}

macro_rules! impl_297 {
    () => {
        deps!();
        impl < 'data , 'file , Elf , R > read :: private :: Sealed for ElfSegment < 'data , 'file , Elf , R > where Elf : FileHeader , R : ReadRef < 'data > , { }
    };
}

impl_297!();