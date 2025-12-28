macro_rules! deps {
    () => {
        ReadRef!();
        MachOSymbolTable!();
        MachHeader!();
    };
}

macro_rules! impl_603 {
    () => {
        deps!();
        impl < 'data , 'file , Mach , R > read :: private :: Sealed for MachOSymbolTable < 'data , 'file , Mach , R > where Mach : MachHeader , R : ReadRef < 'data > , { }
    };
}

impl_603!();