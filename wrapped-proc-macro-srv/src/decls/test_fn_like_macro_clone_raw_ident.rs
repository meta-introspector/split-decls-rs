macro_rules! test_fn_like_macro_clone_raw_ident {
    () => {
        # [test] fn test_fn_like_macro_clone_raw_ident () { assert_expand ("fn_like_clone_tokens" , "r#async" , expect ! [[r#"
            SUBTREE $$ 1 1
              IDENT   r#async 1



            SUBTREE $$ 1 1
              IDENT   r#async 1"#]] , expect ! [[r#"
            SUBTREE $$ 42:Root[0000, 0]@0..100#ROOT2024 42:Root[0000, 0]@0..100#ROOT2024
              IDENT   r#async 42:Root[0000, 0]@0..7#ROOT2024



            SUBTREE $$ 42:Root[0000, 0]@0..100#ROOT2024 42:Root[0000, 0]@0..100#ROOT2024
              IDENT   r#async 42:Root[0000, 0]@0..7#ROOT2024"#]] ,) ; }
    };
}

test_fn_like_macro_clone_raw_ident!()