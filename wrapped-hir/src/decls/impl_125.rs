macro_rules! deps {
    () => {
        TypeAlias!();
        HasVisibility!();
        AssocItem!();
        Const!();
        Function!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl HasVisibility for AssocItem { fn visibility (& self , db : & dyn HirDatabase) -> Visibility { match self { AssocItem :: Function (f) => f . visibility (db) , AssocItem :: Const (c) => c . visibility (db) , AssocItem :: TypeAlias (t) => t . visibility (db) , } } }
    };
}

impl_125!()