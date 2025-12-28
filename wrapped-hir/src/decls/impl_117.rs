macro_rules! deps {
    () => {
        AssocItem!();
        Const!();
        AsAssocItem!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl AsAssocItem for Const { fn as_assoc_item (self , db : & dyn HirDatabase) -> Option < AssocItem > { as_assoc_item (db , AssocItem :: Const , self . id) } }
    };
}

impl_117!()