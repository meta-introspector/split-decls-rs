macro_rules! type_is_option {
    () => {
        fn type_is_option (ty : & Type) -> bool { type_parameter_of_option (ty) . is_some () }
    };
}

type_is_option!()