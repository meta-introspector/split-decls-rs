macro_rules! deps {
    () => {
        MachHeader!();
        ReadRef!();
        MachOFile!();
    };
}

macro_rules! impl_531 {
    () => {
        deps!();
        impl < 'data , Mach , R > read :: private :: Sealed for MachOFile < 'data , Mach , R > where Mach : MachHeader , R : ReadRef < 'data > , { }
    };
}

impl_531!();