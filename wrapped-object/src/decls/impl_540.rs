macro_rules! deps {
    () => {
        MachOComdat!();
        ReadRef!();
        MachHeader!();
    };
}

macro_rules! impl_540 {
    () => {
        deps!();
        impl < 'data , 'file , Mach , R > read :: private :: Sealed for MachOComdat < 'data , 'file , Mach , R > where Mach : MachHeader , R : ReadRef < 'data > , { }
    };
}

impl_540!();