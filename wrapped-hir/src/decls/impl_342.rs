macro_rules! deps {
    () => {
        TypeAlias!();
        AssocItem!();
        Function!();
        ModuleDef!();
        Const!();
    };
}

macro_rules! impl_342 {
    () => {
        deps!();
        impl From < AssocItem > for ModuleDef { fn from (assoc : AssocItem) -> Self { match assoc { AssocItem :: Function (it) => ModuleDef :: Function (it) , AssocItem :: Const (it) => ModuleDef :: Const (it) , AssocItem :: TypeAlias (it) => ModuleDef :: TypeAlias (it) , } } }
    };
}

impl_342!()