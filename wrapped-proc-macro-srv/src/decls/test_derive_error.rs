macro_rules! test_derive_error {
    () => {
        # [test] fn test_derive_error () { assert_expand ("DeriveError" , r#"struct S;"# , expect ! [[r#"
            SUBTREE $$ 1 1
              IDENT   struct 1
              IDENT   S 1
              PUNCH   ; [alone] 1



            SUBTREE $$ 1 1
              IDENT   compile_error 1
              PUNCH   ! [alone] 1
              SUBTREE () 1 1
                LITERAL Str #[derive(DeriveError)] struct S ; 1
              PUNCH   ; [alone] 1"#]] , expect ! [[r#"
            SUBTREE $$ 42:Root[0000, 0]@0..100#ROOT2024 42:Root[0000, 0]@0..100#ROOT2024
              IDENT   struct 42:Root[0000, 0]@0..6#ROOT2024
              IDENT   S 42:Root[0000, 0]@7..8#ROOT2024
              PUNCH   ; [alone] 42:Root[0000, 0]@8..9#ROOT2024



            SUBTREE $$ 42:Root[0000, 0]@0..100#ROOT2024 42:Root[0000, 0]@0..100#ROOT2024
              IDENT   compile_error 42:Root[0000, 0]@0..100#ROOT2024
              PUNCH   ! [alone] 42:Root[0000, 0]@0..100#ROOT2024
              SUBTREE () 42:Root[0000, 0]@0..100#ROOT2024 42:Root[0000, 0]@0..100#ROOT2024
                LITERAL Str #[derive(DeriveError)] struct S ; 42:Root[0000, 0]@0..100#ROOT2024
              PUNCH   ; [alone] 42:Root[0000, 0]@0..100#ROOT2024"#]] ,) ; }
    };
}

test_derive_error!();