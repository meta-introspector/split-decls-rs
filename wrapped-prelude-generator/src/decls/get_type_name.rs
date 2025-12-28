macro_rules! get_type_name {
    () => {
        fn get_type_name (ty : & Box < Type >) -> String { if let Type :: Path (type_path) = & * * ty { if let Some (segment) = type_path . path . segments . last () { return segment . ident . to_string () ; } } String :: new () }
    };
}

get_type_name!()