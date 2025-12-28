macro_rules! deps {
    () => {
        TypeAlias!();
        AssocItem!();
        AsAssocItem!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl AsAssocItem for TypeAlias { fn as_assoc_item (self , db : & dyn HirDatabase) -> Option < AssocItem > { as_assoc_item (db , AssocItem :: TypeAlias , self . id) } }
    };
}

impl_118!()