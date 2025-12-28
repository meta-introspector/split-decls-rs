macro_rules! InputQuery {
    () => {
        pub (crate) struct InputQuery { pub (crate) signature : syn :: Signature , pub (crate) create_data_ident : Ident , }
    };
}

InputQuery!();