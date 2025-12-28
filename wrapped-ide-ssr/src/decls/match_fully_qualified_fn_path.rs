macro_rules! match_fully_qualified_fn_path {
    () => {
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

match_fully_qualified_fn_path!()