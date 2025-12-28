macro_rules! deps {
    () => {
        ExternAssocItem!();
        Static!();
        AsExternAssocItem!();
    };
}

macro_rules! impl_326 {
    () => {
        deps!();
        impl AsExternAssocItem for Static { fn as_extern_assoc_item (self , db : & dyn HirDatabase) -> Option < ExternAssocItem > { as_extern_assoc_item (db , ExternAssocItem :: Static , self . id) } }
    };
}

impl_326!()