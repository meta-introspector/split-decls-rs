// Generated macro for match_fully_qualified_fn_path (function)
macro_rules! Depcrate_testsmatch_fully_qualified_fn_path {
() => {
// Module: crate::tests
// Provides: {"match_fully_qualified_fn_path"}
// Dependencies: {}
# [test] fn match_fully_qualified_fn_path () { let code = r#"
        mod a {
            pub(crate) mod b {
                pub(crate) fn c(_: i32) {}
            }
        }
        use a::b::c;
        fn f1() {
            c(42);
        }
        "# ; assert_matches ("a::b::c($a)" , code , & ["c(42)"]) ; }
};
}
