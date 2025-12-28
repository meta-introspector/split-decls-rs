macro_rules! deps {
    () => {
        ModuleDef!();
        Function!();
        TypeAlias!();
        AsAssocItem!();
        Const!();
        AssocItem!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl AsAssocItem for ModuleDef { fn as_assoc_item (self , db : & dyn HirDatabase) -> Option < AssocItem > { match self { ModuleDef :: Function (it) => it . as_assoc_item (db) , ModuleDef :: Const (it) => it . as_assoc_item (db) , ModuleDef :: TypeAlias (it) => it . as_assoc_item (db) , _ => None , } } }
    };
}

impl_119!()