macro_rules! deps {
    () => {
        ToNav!();
        UpmappingResult!();
        NavigationTarget!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl ToNav for hir :: Local { fn to_nav (& self , db : & RootDatabase) -> UpmappingResult < NavigationTarget > { self . primary_source (db) . to_nav (db) } }
    };
}

impl_46!();