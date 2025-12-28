macro_rules! deps {
    () => {
        Function!();
        AsExternAssocItem!();
        ExternAssocItem!();
    };
}

macro_rules! impl_325 {
    () => {
        deps!();
        impl AsExternAssocItem for Function { fn as_extern_assoc_item (self , db : & dyn HirDatabase) -> Option < ExternAssocItem > { as_extern_assoc_item (db , ExternAssocItem :: Function , self . id) } }
    };
}

impl_325!()