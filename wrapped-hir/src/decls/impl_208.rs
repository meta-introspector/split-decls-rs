macro_rules! deps {
    () => {
        Crate!();
        Macro!();
        HasCrate!();
    };
}

macro_rules! impl_208 {
    () => {
        deps!();
        impl HasCrate for Macro { fn krate (& self , db : & dyn HirDatabase) -> Crate { self . module (db) . krate () } }
    };
}

impl_208!()