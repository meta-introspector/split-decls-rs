macro_rules! deps {
    () => {
        ImportAliasDisplay!();
        ImportAlias!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl fmt :: Display for ImportAliasDisplay < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . value { ImportAlias :: Underscore => f . write_str ("_") , ImportAlias :: Alias (name) => fmt :: Display :: fmt (& name . display_no_db (self . edition) , f) , } } }
    };
}

impl_169!();