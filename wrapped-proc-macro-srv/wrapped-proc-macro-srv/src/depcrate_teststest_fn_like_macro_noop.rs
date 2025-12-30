// Generated macro for test_fn_like_macro_noop (function)
macro_rules! Depcrate_teststest_fn_like_macro_noop {
() => {
// Module: crate::tests
// Provides: {"test_fn_like_macro_noop"}
// Dependencies: {}
# [test] fn test_fn_like_macro_noop () { assert_expand ("fn_like_noop" , r#"ident, 0, 1, []"# , expect ! [[r#"
            SUBTREE $$ 1 1
              IDENT   ident 1
              PUNCH   , [alone] 1
              LITERAL Integer 0 1
              PUNCH   , [alone] 1
              LITERAL Integer 1 1
              PUNCH   , [alone] 1
              SUBTREE [] 1 1



            SUBTREE $$ 1 1
              IDENT   ident 1
              PUNCH   , [alone] 1
              LITERAL Integer 0 1
              PUNCH   , [alone] 1
              LITERAL Integer 1 1
              PUNCH   , [alone] 1
              SUBTREE [] 1 1"#]] , expect ! [[r#"
            SUBTREE $$ 42:Root[0000, 0]@0..100#ROOT2024 42:Root[0000, 0]@0..100#ROOT2024
              IDENT   ident 42:Root[0000, 0]@0..5#ROOT2024
              PUNCH   , [alone] 42:Root[0000, 0]@5..6#ROOT2024
              LITERAL Integer 0 42:Root[0000, 0]@7..8#ROOT2024
              PUNCH   , [alone] 42:Root[0000, 0]@8..9#ROOT2024
              LITERAL Integer 1 42:Root[0000, 0]@10..11#ROOT2024
              PUNCH   , [alone] 42:Root[0000, 0]@11..12#ROOT2024
              SUBTREE [] 42:Root[0000, 0]@13..14#ROOT2024 42:Root[0000, 0]@14..15#ROOT2024



            SUBTREE $$ 42:Root[0000, 0]@0..100#ROOT2024 42:Root[0000, 0]@0..100#ROOT2024
              IDENT   ident 42:Root[0000, 0]@0..5#ROOT2024
              PUNCH   , [alone] 42:Root[0000, 0]@5..6#ROOT2024
              LITERAL Integer 0 42:Root[0000, 0]@7..8#ROOT2024
              PUNCH   , [alone] 42:Root[0000, 0]@8..9#ROOT2024
              LITERAL Integer 1 42:Root[0000, 0]@10..11#ROOT2024
              PUNCH   , [alone] 42:Root[0000, 0]@11..12#ROOT2024
              SUBTREE [] 42:Root[0000, 0]@13..14#ROOT2024 42:Root[0000, 0]@14..15#ROOT2024"#]] ,) ; }
};
}
