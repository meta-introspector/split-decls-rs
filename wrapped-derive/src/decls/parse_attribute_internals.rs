macro_rules! deps {
    () => {
        FieldConstructor!();
    };
}

macro_rules! parse_attribute_internals {
    () => {
        deps!();
        fn parse_attribute_internals (meta_list : & MetaList) -> Result < FieldConstructor > { let mut tokens_iter = meta_list . tokens . clone () . into_iter () ; let token = tokens_iter . next () . ok_or_else (| | { let msg = format ! ("#[{ARBITRARY_ATTRIBUTE_NAME}] cannot be empty.") ; syn :: Error :: new (meta_list . span () , msg) }) ? ; match token . to_string () . as_ref () { "default" => Ok (FieldConstructor :: Default) , "with" => { let func_path = parse_assigned_value ("with" , tokens_iter , meta_list . span ()) ? ; Ok (FieldConstructor :: With (func_path)) } "value" => { let value = parse_assigned_value ("value" , tokens_iter , meta_list . span ()) ? ; Ok (FieldConstructor :: Value (value)) } _ => { let msg = format ! ("Unknown option for #[{ARBITRARY_ATTRIBUTE_NAME}]: `{token}`") ; Err (syn :: Error :: new (token . span () , msg)) } } }
    };
}

parse_attribute_internals!()