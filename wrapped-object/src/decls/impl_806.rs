macro_rules! deps {
    () => {
        FileHeader!();
        ReadRef!();
        StringTable!();
        SymbolTable!();
    };
}

macro_rules! impl_806 {
    () => {
        deps!();
        impl < 'data , Xcoff , R > Default for SymbolTable < 'data , Xcoff , R > where Xcoff : FileHeader , R : ReadRef < 'data > , { fn default () -> Self { Self { symbols : & [] , strings : StringTable :: default () , header : PhantomData , } } }
    };
}

impl_806!()