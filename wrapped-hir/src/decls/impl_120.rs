macro_rules! deps {
    () => {
        AssocItem!();
        Function!();
        Static!();
        DefWithBody!();
        Variant!();
        AsAssocItem!();
        Const!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl AsAssocItem for DefWithBody { fn as_assoc_item (self , db : & dyn HirDatabase) -> Option < AssocItem > { match self { DefWithBody :: Function (it) => it . as_assoc_item (db) , DefWithBody :: Const (it) => it . as_assoc_item (db) , DefWithBody :: Static (_) | DefWithBody :: Variant (_) => None , } } }
    };
}

impl_120!()