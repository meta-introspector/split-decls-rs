macro_rules! is_generic_ty {
    () => {
        fn is_generic_ty (ty : & Type , name : & str) -> bool { subty_if_name (ty , name) . is_some () }
    };
}

is_generic_ty!()