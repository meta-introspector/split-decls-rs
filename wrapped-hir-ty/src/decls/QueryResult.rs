macro_rules! QueryResult {
    () => {
        pub type QueryResult < 'db > = rustc_type_ir :: solve :: QueryResult < DbInterner < 'db > > ;
    };
}

QueryResult!()