macro_rules! match_path {
    () => {
        # [test] fn match_path () { let code = r#"
        mod foo {
            pub(crate) fn bar() {}
        }
        fn f() {foo::bar(42)}"# ; assert_matches ("foo::bar" , code , & ["foo::bar"]) ; assert_matches ("$a::bar" , code , & ["foo::bar"]) ; assert_matches ("foo::$b" , code , & ["foo::bar"]) ; }
    };
}

match_path!();