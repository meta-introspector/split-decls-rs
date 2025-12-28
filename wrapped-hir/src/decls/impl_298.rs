macro_rules! deps {
    () => {
        Const!();
        HasVisibility!();
    };
}

macro_rules! impl_298 {
    () => {
        deps!();
        impl HasVisibility for Const { fn visibility (& self , db : & dyn HirDatabase) -> Visibility { db . assoc_visibility (self . id . into ()) } }
    };
}

impl_298!()