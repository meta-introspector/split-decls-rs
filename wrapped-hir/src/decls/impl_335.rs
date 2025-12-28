macro_rules! deps {
    () => {
        ModuleDef!();
        Function!();
        AssocItem!();
        TypeAlias!();
        AsAssocItem!();
        Const!();
    };
}

macro_rules! impl_335 {
    () => {
        deps!();
        impl AsAssocItem for ModuleDef { fn as_assoc_item (self , db : & dyn HirDatabase) -> Option < AssocItem > { match self { ModuleDef :: Function (it) => it . as_assoc_item (db) , ModuleDef :: Const (it) => it . as_assoc_item (db) , ModuleDef :: TypeAlias (it) => it . as_assoc_item (db) , _ => None , } } }
    };
}

impl_335!()