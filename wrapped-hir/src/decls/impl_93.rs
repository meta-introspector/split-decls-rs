macro_rules! deps {
    () => {
        TypeAlias!();
        HasVisibility!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl HasVisibility for TypeAlias { fn visibility (& self , db : & dyn HirDatabase) -> Visibility { db . assoc_visibility (self . id . into ()) } }
    };
}

impl_93!()