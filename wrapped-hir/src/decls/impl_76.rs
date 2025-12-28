macro_rules! deps {
    () => {
        Function!();
        HasVisibility!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl HasVisibility for Function { fn visibility (& self , db : & dyn HirDatabase) -> Visibility { db . assoc_visibility (self . id . into ()) } }
    };
}

impl_76!()