macro_rules! deps {
    () => {
        MachOSection!();
        ReadRef!();
        MachHeader!();
    };
}

macro_rules! impl_589 {
    () => {
        deps!();
        impl < 'data , 'file , Mach , R > read :: private :: Sealed for MachOSection < 'data , 'file , Mach , R > where Mach : MachHeader , R : ReadRef < 'data > , { }
    };
}

impl_589!();