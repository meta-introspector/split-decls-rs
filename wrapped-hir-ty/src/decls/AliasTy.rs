macro_rules! AliasTy {
    () => {
        pub type AliasTy < 'db > = rustc_type_ir :: AliasTy < DbInterner < 'db > > ;
    };
}

AliasTy!()