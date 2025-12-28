macro_rules! deps {
    () => {
        SymbolTable!();
        ReadRef!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        impl < 'data , 'file , R : ReadRef < 'data > > read :: private :: Sealed for SymbolTable < 'data , 'file , R > { }
    };
}

impl_156!()