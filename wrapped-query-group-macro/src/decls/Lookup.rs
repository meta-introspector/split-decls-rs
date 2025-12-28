macro_rules! Lookup {
    () => {
        pub (crate) struct Lookup { pub (crate) signature : syn :: Signature , pub (crate) pat_and_tys : Vec < PatType > , pub (crate) return_ty : Type , pub (crate) interned_struct_path : Path , }
    };
}

Lookup!()