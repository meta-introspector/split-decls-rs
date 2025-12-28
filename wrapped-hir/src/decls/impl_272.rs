macro_rules! deps {
    () => {
        Variant!();
        HasVisibility!();
    };
}

macro_rules! impl_272 {
    () => {
        deps!();
        # [doc = " Variants inherit visibility from the parent enum."] impl HasVisibility for Variant { fn visibility (& self , db : & dyn HirDatabase) -> Visibility { self . parent_enum (db) . visibility (db) } }
    };
}

impl_272!();