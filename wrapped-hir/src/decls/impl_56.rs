macro_rules! deps {
    () => {
        HasVisibility!();
        Variant!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        # [doc = " Variants inherit visibility from the parent enum."] impl HasVisibility for Variant { fn visibility (& self , db : & dyn HirDatabase) -> Visibility { self . parent_enum (db) . visibility (db) } }
    };
}

impl_56!()