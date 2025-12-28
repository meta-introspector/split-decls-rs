macro_rules! deps {
    () => {
        GeneratorResult!();
    };
}

macro_rules! extract_input_args {
    () => {
        deps!();
        pub fn extract_input_args < T : FromMeta + Default > (crate_name : & proc_macro2 :: TokenStream , method : & mut ImplItemFn ,) -> GeneratorResult < Vec < (PatIdent , Type , T) > > { let mut args = Vec :: new () ; let mut create_ctx = true ; if method . sig . inputs . is_empty () { return Err (Error :: new_spanned (& method . sig , "The self receiver must be the first parameter." ,) . into ()) ; } for (idx , arg) in method . sig . inputs . iter_mut () . enumerate () { if let FnArg :: Receiver (receiver) = arg { if idx != 0 { return Err (Error :: new_spanned (receiver , "The self receiver must be the first parameter." ,) . into ()) ; } } else if let FnArg :: Typed (pat) = arg { if idx == 0 { return Err (Error :: new_spanned (pat , "The self receiver must be the first parameter." ,) . into ()) ; } match (& * pat . pat , & * pat . ty) { (Pat :: Ident (arg_ident) , Type :: Reference (TypeReference { elem , .. })) => { if let Type :: Path (path) = elem . as_ref () { if idx != 1 || path . path . segments . last () . unwrap () . ident != "Context" { args . push ((arg_ident . clone () , pat . ty . as_ref () . clone () , parse_graphql_attrs :: < T > (& pat . attrs) ? . unwrap_or_default () ,)) ; } else { create_ctx = false ; } } } (Pat :: Ident (arg_ident) , ty) => { args . push ((arg_ident . clone () , ty . clone () , parse_graphql_attrs :: < T > (& pat . attrs) ? . unwrap_or_default () ,)) ; remove_graphql_attrs (& mut pat . attrs) ; } _ => { return Err (Error :: new_spanned (arg , "Invalid argument type.") . into ()) ; } } } } if create_ctx { let arg = syn :: parse2 :: < FnArg > (quote ! { _ : &# crate_name :: Context <'_ > }) . unwrap () ; method . sig . inputs . insert (1 , arg) ; } Ok (args) }
    };
}

extract_input_args!();