macro_rules! test_fn_like_mk_literals {
    () => {
        # [test] fn test_fn_like_mk_literals () { assert_expand ("fn_like_mk_literals" , r#""# , expect ! [[r#"
            SUBTREE $$ 1 1



            SUBTREE $$ 1 1
              LITERAL ByteStr byte_string 1
              LITERAL Char c 1
              LITERAL Str string 1
              LITERAL Str -string 1
              LITERAL CStr cstring 1
              LITERAL Float 3.14f64 1
              PUNCH   - [alone] 1
              LITERAL Float 3.14f64 1
              LITERAL Float 3.14 1
              PUNCH   - [alone] 1
              LITERAL Float 3.14 1
              LITERAL Integer 123i64 1
              PUNCH   - [alone] 1
              LITERAL Integer 123i64 1
              LITERAL Integer 123 1
              PUNCH   - [alone] 1
              LITERAL Integer 123 1"#]] , expect ! [[r#"
            SUBTREE $$ 42:Root[0000, 0]@0..100#ROOT2024 42:Root[0000, 0]@0..100#ROOT2024



            SUBTREE $$ 42:Root[0000, 0]@0..100#ROOT2024 42:Root[0000, 0]@0..100#ROOT2024
              LITERAL ByteStr byte_string 42:Root[0000, 0]@0..100#ROOT2024
              LITERAL Char c 42:Root[0000, 0]@0..100#ROOT2024
              LITERAL Str string 42:Root[0000, 0]@0..100#ROOT2024
              LITERAL Str -string 42:Root[0000, 0]@0..100#ROOT2024
              LITERAL CStr cstring 42:Root[0000, 0]@0..100#ROOT2024
              LITERAL Float 3.14f64 42:Root[0000, 0]@0..100#ROOT2024
              PUNCH   - [alone] 42:Root[0000, 0]@0..100#ROOT2024
              LITERAL Float 3.14f64 42:Root[0000, 0]@0..100#ROOT2024
              LITERAL Float 3.14 42:Root[0000, 0]@0..100#ROOT2024
              PUNCH   - [alone] 42:Root[0000, 0]@0..100#ROOT2024
              LITERAL Float 3.14 42:Root[0000, 0]@0..100#ROOT2024
              LITERAL Integer 123i64 42:Root[0000, 0]@0..100#ROOT2024
              PUNCH   - [alone] 42:Root[0000, 0]@0..100#ROOT2024
              LITERAL Integer 123i64 42:Root[0000, 0]@0..100#ROOT2024
              LITERAL Integer 123 42:Root[0000, 0]@0..100#ROOT2024
              PUNCH   - [alone] 42:Root[0000, 0]@0..100#ROOT2024
              LITERAL Integer 123 42:Root[0000, 0]@0..100#ROOT2024"#]] ,) ; }
    };
}

test_fn_like_mk_literals!();