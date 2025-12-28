macro_rules! parse_nested_meta {
    () => {
        macro_rules ! parse_nested_meta { ($ ty : ty , $ args : expr) => { { let meta = match darling :: ast :: NestedMeta :: parse_meta_list (proc_macro2 :: TokenStream :: from ($ args ,)) { Ok (v) => v , Err (e) => { return TokenStream :: from (darling :: Error :: from (e) . write_errors ()) ; } } ; match <$ ty >:: from_list (& meta) { Ok (object_args) => object_args , Err (err) => return TokenStream :: from (err . write_errors ()) , } } } ; }
    };
}

parse_nested_meta!();