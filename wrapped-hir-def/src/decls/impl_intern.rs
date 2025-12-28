macro_rules! deps {
    () => {
        DefDatabase!();
    };
}

macro_rules! impl_intern {
    () => {
        deps!();
        macro_rules ! impl_intern { ($ id : ident , $ loc : ident , $ intern : ident , $ lookup : ident) => { impl_intern_key ! ($ id , $ loc) ; impl_intern_lookup ! (DefDatabase , $ id , $ loc , $ intern , $ lookup) ; } ; }
    };
}

impl_intern!()