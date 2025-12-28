macro_rules! test_fn_like_fn_like_span_join {
    () => {
        # [test] fn test_fn_like_fn_like_span_join () { assert_expand ("fn_like_span_join" , "foo     bar" , expect ! [[r#"
            SUBTREE $$ 1 1
              IDENT   foo 1
              IDENT   bar 1



            SUBTREE $$ 1 1
              IDENT   r#joined 1"#]] , expect ! [[r#"
            SUBTREE $$ 42:Root[0000, 0]@0..100#ROOT2024 42:Root[0000, 0]@0..100#ROOT2024
              IDENT   foo 42:Root[0000, 0]@0..3#ROOT2024
              IDENT   bar 42:Root[0000, 0]@8..11#ROOT2024



            SUBTREE $$ 42:Root[0000, 0]@0..100#ROOT2024 42:Root[0000, 0]@0..100#ROOT2024
              IDENT   r#joined 42:Root[0000, 0]@0..11#ROOT2024"#]] ,) ; }
    };
}

test_fn_like_fn_like_span_join!()