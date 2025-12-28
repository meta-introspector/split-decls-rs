macro_rules! deps {
    () => {
        NavigationTarget!();
        UpmappingResult!();
    };
}

macro_rules! ToNav {
    () => {
        deps!();
        pub (crate) trait ToNav { fn to_nav (& self , db : & RootDatabase) -> UpmappingResult < NavigationTarget > ; }
    };
}

ToNav!()