macro_rules! deps {
    () => {
        AssocItem!();
    };
}

macro_rules! AsAssocItem {
    () => {
        deps!();
        pub trait AsAssocItem { fn as_assoc_item (self , db : & dyn HirDatabase) -> Option < AssocItem > ; }
    };
}

AsAssocItem!()