macro_rules! deps {
    () => {
        OutputType!();
        GeneratorResult!();
        FieldResolver!();
        FieldResolverParameter!();
        Object!();
        ObjectField!();
    };
}

macro_rules! generate_field_resolver_method {
    () => {
        deps!();
        fn generate_field_resolver_method (crate_name : & proc_macro2 :: TokenStream , object_args : & args :: Object , method_args : & args :: ObjectField , field : & FieldResolver ,) -> GeneratorResult < (Ident , proc_macro2 :: TokenStream) > { let FieldResolver { resolver_fn_ident : resolver_ident , params , cfg_attrs , } = field ; let extract_params = params . iter () . map (| param | generate_parameter_extraction (crate_name , param)) . collect :: < Result < Vec < _ > , _ > > () ? ; let use_params = params . iter () . map (| FieldResolverParameter { ident : PatIdent { ident , .. } , .. } | ident ,) ; let guard_map_err = quote ! { . map_err (| err | err . into_server_error (ctx . item . pos)) } ; let guard = match method_args . guard . as_ref () . or (object_args . guard . as_ref ()) { Some (code) => Some (generate_guards (crate_name , code , guard_map_err) ?) , None => None , } ; let mut resolve_fn_name = syn :: parse_str :: < Ident > (& format ! ("__{}_resolver" , field . resolver_fn_ident . unraw ())) ? ; resolve_fn_name . set_span (Span :: call_site ()) ; let function = quote ! { # [doc (hidden)] # (# cfg_attrs) * # [allow (non_snake_case)] async fn # resolve_fn_name (& self , ctx : &# crate_name :: Context <'_ >) -> # crate_name :: ServerResult <:: std :: option :: Option <# crate_name :: Value >> { let f = async { # (# extract_params) * # guard let res = self .# resolver_ident (ctx , # (# use_params) ,*) . await ; res . map_err (| err | :: std :: convert :: Into ::<# crate_name :: Error >:: into (err) . into_server_error (ctx . item . pos)) } ; let obj = f . await . map_err (| err | ctx . set_error_path (err)) ?; let ctx_obj = ctx . with_selection_set (& ctx . item . node . selection_set) ; return # crate_name :: OutputType :: resolve (& obj , & ctx_obj , ctx . item) . await . map (:: std :: option :: Option :: Some) ; } } ; Ok ((resolve_fn_name , function)) }
    };
}

generate_field_resolver_method!()