macro_rules! deps {
    () => {
        CoffHeader!();
        StringTable!();
        SymbolTable!();
        ReadRef!();
    };
}

macro_rules! impl_228 {
    () => {
        deps!();
        impl < 'data , R : ReadRef < 'data > , Coff : CoffHeader > Default for SymbolTable < 'data , R , Coff > { fn default () -> Self { Self { symbols : & [] , strings : StringTable :: default () , } } }
    };
}

impl_228!();