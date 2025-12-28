macro_rules! deps {
    () => {
        GeneratorResult!();
        FieldResolverParameter!();
    };
}

macro_rules! generate_parameter_extraction {
    () => {
        deps!();
        fn generate_parameter_extraction (crate_name : & proc_macro2 :: TokenStream , parameter : & FieldResolverParameter ,) -> GeneratorResult < proc_macro2 :: TokenStream > { let FieldResolverParameter { ty , name , default , process_with , validator , ident , } = parameter ; let default = match default { Some (default) => { quote ! { :: std :: option :: Option :: Some (|| -> # ty { # default }) } } None => quote ! { :: std :: option :: Option :: None } , } ; let process_with = match process_with . as_ref () { Some (fn_path) => quote ! { # fn_path (& mut # ident) ; } , None => Default :: default () , } ; let validators = (* validator) . clone () . unwrap_or_default () . create_validators (crate_name , quote ! (&# ident) , Some (quote ! (. map_err (| err | err . into_server_error (__pos)))) ,) ? ; let mut non_mut_ident = ident . clone () ; non_mut_ident . mutability = None ; let ident = & ident . ident ; Ok (quote ! { # [allow (non_snake_case , unused_variables , unused_mut)] let (__pos , mut # non_mut_ident) = ctx . param_value ::<# ty > (# name , # default) ?; # process_with # validators # [allow (non_snake_case , unused_variables)] let # ident = # non_mut_ident ; }) }
    };
}

generate_parameter_extraction!()