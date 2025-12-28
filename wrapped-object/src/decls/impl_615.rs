macro_rules! deps {
    () => {
        MachOSymbol!();
        MachHeader!();
        ReadRef!();
    };
}

macro_rules! impl_615 {
    () => {
        deps!();
        impl < 'data , 'file , Mach , R > read :: private :: Sealed for MachOSymbol < 'data , 'file , Mach , R > where Mach : MachHeader , R : ReadRef < 'data > , { }
    };
}

impl_615!()