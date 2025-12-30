// Generated macro for test_attr_macro (function)
macro_rules! Depcrate_teststest_attr_macro {
() => {
// Module: crate::tests
// Provides: {"test_attr_macro"}
// Dependencies: {}
# [test] fn test_attr_macro () { assert_expand_attr ("attr_error" , r#"mod m {}"# , r#"some arguments"# , expect ! [[r#"
            SUBTREE $$ 1 1
              IDENT   mod 1
              IDENT   m 1
              SUBTREE {} 1 1

            SUBTREE $$ 1 1
              IDENT   some 1
              IDENT   arguments 1

            SUBTREE $$ 1 1
              IDENT   compile_error 1
              PUNCH   ! [alone] 1
              SUBTREE () 1 1
                LITERAL Str #[attr_error(some arguments)] mod m {} 1
              PUNCH   ; [alone] 1"#]] , expect ! [[r#"
            SUBTREE $$ 42:Root[0000, 0]@0..100#ROOT2024 42:Root[0000, 0]@0..100#ROOT2024
              IDENT   mod 42:Root[0000, 0]@0..3#ROOT2024
              IDENT   m 42:Root[0000, 0]@4..5#ROOT2024
              SUBTREE {} 42:Root[0000, 0]@6..7#ROOT2024 42:Root[0000, 0]@7..8#ROOT2024

            SUBTREE $$ 42:Root[0000, 0]@0..100#ROOT2024 42:Root[0000, 0]@0..100#ROOT2024
              IDENT   some 42:Root[0000, 0]@0..4#ROOT2024
              IDENT   arguments 42:Root[0000, 0]@5..14#ROOT2024

            SUBTREE $$ 42:Root[0000, 0]@0..100#ROOT2024 42:Root[0000, 0]@0..100#ROOT2024
              IDENT   compile_error 42:Root[0000, 0]@0..100#ROOT2024
              PUNCH   ! [alone] 42:Root[0000, 0]@0..100#ROOT2024
              SUBTREE () 42:Root[0000, 0]@0..100#ROOT2024 42:Root[0000, 0]@0..100#ROOT2024
                LITERAL Str #[attr_error(some arguments)] mod m {} 42:Root[0000, 0]@0..100#ROOT2024
              PUNCH   ; [alone] 42:Root[0000, 0]@0..100#ROOT2024"#]] ,) ; }
};
}
