macro_rules! TypeError {
    () => {
        pub type TypeError < 'db > = rustc_type_ir :: error :: TypeError < DbInterner < 'db > > ;
    };
}

TypeError!()