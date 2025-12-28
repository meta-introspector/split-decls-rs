macro_rules! InputSetterWithDurability {
    () => {
        pub (crate) struct InputSetterWithDurability { pub (crate) signature : syn :: Signature , pub (crate) return_type : syn :: Type , pub (crate) create_data_ident : Ident , }
    };
}

InputSetterWithDurability!()