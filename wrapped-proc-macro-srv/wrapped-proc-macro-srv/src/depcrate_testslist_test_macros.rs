// Generated macro for list_test_macros (function)
macro_rules! Depcrate_testslist_test_macros {
() => {
// Module: crate::tests
// Provides: {"list_test_macros"}
// Dependencies: {}
# [doc = " Tests that we find and classify all proc macros correctly."] # [test] fn list_test_macros () { let res = list () . join ("\n") ; expect ! [[r#"
        fn_like_noop [Bang]
        fn_like_panic [Bang]
        fn_like_error [Bang]
        fn_like_clone_tokens [Bang]
        fn_like_mk_literals [Bang]
        fn_like_mk_idents [Bang]
        fn_like_span_join [Bang]
        fn_like_span_ops [Bang]
        attr_noop [Attr]
        attr_panic [Attr]
        attr_error [Attr]
        DeriveEmpty [CustomDerive]
        DerivePanic [CustomDerive]
        DeriveError [CustomDerive]"#]] . assert_eq (& res) ; }
};
}
