macro_rules! unoptional_type {
    () => {
        fn unoptional_type (ty : & Type) -> TokenStream { let unoptional = type_parameter_of_option (ty) . unwrap_or (ty) ; quote ! (# unoptional) }
    };
}

unoptional_type!()