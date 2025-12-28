macro_rules! Intern {
    () => {
        pub (crate) struct Intern { pub (crate) signature : syn :: Signature , pub (crate) pat_and_tys : Vec < PatType > , pub (crate) interned_struct_path : Path , }
    };
}

Intern!()