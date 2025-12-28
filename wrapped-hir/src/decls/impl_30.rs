macro_rules! deps {
    () => {
        TypeAlias!();
        Const!();
        AssocItem!();
        Function!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl From < AssocItemId > for AssocItem { fn from (def : AssocItemId) -> Self { match def { AssocItemId :: FunctionId (it) => AssocItem :: Function (it . into ()) , AssocItemId :: TypeAliasId (it) => AssocItem :: TypeAlias (it . into ()) , AssocItemId :: ConstId (it) => AssocItem :: Const (it . into ()) , } } }
    };
}

impl_30!();