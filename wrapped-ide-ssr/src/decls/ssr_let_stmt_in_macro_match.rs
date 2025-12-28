macro_rules! ssr_let_stmt_in_macro_match {
    () => {
        # [test] fn ssr_let_stmt_in_macro_match () { assert_matches ("let a = 0" , r#"
            macro_rules! m1 { ($a:stmt) => {$a}; }
            fn f() {m1!{ let a = 0 };}"# , & ["leta=0"] ,) ; }
    };
}

ssr_let_stmt_in_macro_match!();