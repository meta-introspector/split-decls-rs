macro_rules! deps {
    () => {
        HasVisibility!();
        Const!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl HasVisibility for Const { fn visibility (& self , db : & dyn HirDatabase) -> Visibility { db . assoc_visibility (self . id . into ()) } }
    };
}

impl_82!()