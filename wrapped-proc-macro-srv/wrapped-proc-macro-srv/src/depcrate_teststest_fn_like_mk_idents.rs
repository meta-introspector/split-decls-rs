// Generated macro for test_fn_like_mk_idents (function)
macro_rules! Depcrate_teststest_fn_like_mk_idents {
() => {
// Module: crate::tests
// Provides: {"test_fn_like_mk_idents"}
// Dependencies: {}
# [test] fn test_fn_like_mk_idents () { assert_expand ("fn_like_mk_idents" , r#""# , expect ! [[r#"
            SUBTREE $$ 1 1



            SUBTREE $$ 1 1
              IDENT   standard 1
              IDENT   r#raw 1"#]] , expect ! [[r#"
            SUBTREE $$ 42:Root[0000, 0]@0..100#ROOT2024 42:Root[0000, 0]@0..100#ROOT2024



            SUBTREE $$ 42:Root[0000, 0]@0..100#ROOT2024 42:Root[0000, 0]@0..100#ROOT2024
              IDENT   standard 42:Root[0000, 0]@0..100#ROOT2024
              IDENT   r#raw 42:Root[0000, 0]@0..100#ROOT2024"#]] ,) ; }
};
}
