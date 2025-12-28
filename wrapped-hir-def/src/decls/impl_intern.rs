macro_rules! impl_intern {
    () => {
        macro_rules ! impl_intern { ($ id : ident , $ loc : ident , $ intern : ident , $ lookup : ident) => { impl_intern_key ! ($ id , $ loc) ; impl_intern_lookup ! (DefDatabase , $ id , $ loc , $ intern , $ lookup) ; } ; }
    };
}

impl_intern!()