macro_rules! deps {
    () => {
        AsAssocItem!();
        Function!();
        AssocItem!();
        Variant!();
        Static!();
        Const!();
        DefWithBody!();
    };
}

macro_rules! impl_336 {
    () => {
        deps!();
        impl AsAssocItem for DefWithBody { fn as_assoc_item (self , db : & dyn HirDatabase) -> Option < AssocItem > { match self { DefWithBody :: Function (it) => it . as_assoc_item (db) , DefWithBody :: Const (it) => it . as_assoc_item (db) , DefWithBody :: Static (_) | DefWithBody :: Variant (_) => None , } } }
    };
}

impl_336!();