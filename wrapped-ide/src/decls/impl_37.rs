macro_rules! deps {
    () => {
        UpmappingResult!();
        NavigationTarget!();
        ToNav!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl ToNav for hir :: Crate { fn to_nav (& self , db : & RootDatabase) -> UpmappingResult < NavigationTarget > { self . root_module () . to_nav (db) } }
    };
}

impl_37!()