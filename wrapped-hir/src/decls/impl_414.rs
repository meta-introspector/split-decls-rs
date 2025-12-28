macro_rules! deps {
    () => {
        Crate!();
        AssocItem!();
        HasCrate!();
    };
}

macro_rules! impl_414 {
    () => {
        deps!();
        impl HasCrate for AssocItem { fn krate (& self , db : & dyn HirDatabase) -> Crate { self . module (db) . krate () } }
    };
}

impl_414!();