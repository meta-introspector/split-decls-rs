macro_rules! InputSetter {
    () => {
        pub (crate) struct InputSetter { pub (crate) signature : syn :: Signature , pub (crate) return_type : syn :: Type , pub (crate) create_data_ident : Ident , }
    };
}

InputSetter!();