macro_rules! deps {
    () => {
        TypeAlias!();
        AsExternAssocItem!();
        ExternAssocItem!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl AsExternAssocItem for TypeAlias { fn as_extern_assoc_item (self , db : & dyn HirDatabase) -> Option < ExternAssocItem > { as_extern_assoc_item (db , ExternAssocItem :: TypeAlias , self . id) } }
    };
}

impl_111!()