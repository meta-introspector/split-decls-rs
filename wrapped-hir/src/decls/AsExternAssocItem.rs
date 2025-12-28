macro_rules! deps {
    () => {
        ExternAssocItem!();
    };
}

macro_rules! AsExternAssocItem {
    () => {
        deps!();
        pub trait AsExternAssocItem { fn as_extern_assoc_item (self , db : & dyn HirDatabase) -> Option < ExternAssocItem > ; }
    };
}

AsExternAssocItem!()