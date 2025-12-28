macro_rules! deps {
    () => {
        Result!();
        ReadRef!();
        MachHeader!();
        MachOSymbolIterator!();
    };
}

macro_rules! impl_609 {
    () => {
        deps!();
        impl < 'data , 'file , Mach , R > fmt :: Debug for MachOSymbolIterator < 'data , 'file , Mach , R > where Mach : MachHeader , R : ReadRef < 'data > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("MachOSymbolIterator") . finish () } }
    };
}

impl_609!();