macro_rules! deps {
    () => {
        ReadRef!();
        SymbolTable!();
        MachHeader!();
    };
}

macro_rules! impl_598 {
    () => {
        deps!();
        impl < 'data , Mach : MachHeader , R : ReadRef < 'data > > Default for SymbolTable < 'data , Mach , R > { fn default () -> Self { SymbolTable { symbols : & [] , strings : Default :: default () , } } }
    };
}

impl_598!();