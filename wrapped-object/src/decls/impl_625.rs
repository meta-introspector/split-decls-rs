macro_rules! deps {
    () => {
        MachORelocationIterator!();
        MachHeader!();
        ReadRef!();
        Result!();
    };
}

macro_rules! impl_625 {
    () => {
        deps!();
        impl < 'data , 'file , Mach , R > fmt :: Debug for MachORelocationIterator < 'data , 'file , Mach , R > where Mach : MachHeader , R : ReadRef < 'data > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("MachORelocationIterator") . finish () } }
    };
}

impl_625!();