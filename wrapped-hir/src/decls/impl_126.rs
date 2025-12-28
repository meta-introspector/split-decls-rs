macro_rules! deps {
    () => {
        AssocItem!();
        Const!();
        ModuleDef!();
        Function!();
        TypeAlias!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl From < AssocItem > for ModuleDef { fn from (assoc : AssocItem) -> Self { match assoc { AssocItem :: Function (it) => ModuleDef :: Function (it) , AssocItem :: Const (it) => ModuleDef :: Const (it) , AssocItem :: TypeAlias (it) => ModuleDef :: TypeAlias (it) , } } }
    };
}

impl_126!()