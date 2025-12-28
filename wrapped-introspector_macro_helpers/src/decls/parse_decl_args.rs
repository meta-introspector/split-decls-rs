macro_rules! parse_decl_args {
    () => {
        # [macro_export] macro_rules ! parse_decl_args { ($ attr_token_stream : expr) => { syn :: parse2 ($ attr_token_stream) . unwrap () } ; }
    };
}

parse_decl_args!()