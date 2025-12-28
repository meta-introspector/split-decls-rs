macro_rules! deps {
    () => {
        MachOSectionIterator!();
        MachHeader!();
        ReadRef!();
        Result!();
    };
}

macro_rules! impl_583 {
    () => {
        deps!();
        impl < 'data , 'file , Mach , R > fmt :: Debug for MachOSectionIterator < 'data , 'file , Mach , R > where Mach : MachHeader , R : ReadRef < 'data > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("MachOSectionIterator") . finish () } }
    };
}

impl_583!()