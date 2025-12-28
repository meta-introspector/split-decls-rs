macro_rules! deps {
    () => {
        HasCrate!();
        Crate!();
        Enum!();
    };
}

macro_rules! impl_201 {
    () => {
        deps!();
        impl HasCrate for Enum { fn krate (& self , db : & dyn HirDatabase) -> Crate { self . module (db) . krate () } }
    };
}

impl_201!()