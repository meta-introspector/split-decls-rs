macro_rules! deps {
    () => {
        MachHeader!();
        ReadRef!();
        MachOSegment!();
    };
}

macro_rules! impl_573 {
    () => {
        deps!();
        impl < 'data , 'file , Mach , R > read :: private :: Sealed for MachOSegment < 'data , 'file , Mach , R > where Mach : MachHeader , R : ReadRef < 'data > , { }
    };
}

impl_573!()