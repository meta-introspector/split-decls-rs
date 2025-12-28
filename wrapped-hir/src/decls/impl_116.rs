macro_rules! deps {
    () => {
        Function!();
        AsAssocItem!();
        AssocItem!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        impl AsAssocItem for Function { fn as_assoc_item (self , db : & dyn HirDatabase) -> Option < AssocItem > { as_assoc_item (db , AssocItem :: Function , self . id) } }
    };
}

impl_116!()