macro_rules! ssr_blockexpr_replace_trailing_expr_with_stmt {
    () => {
        # [test] fn ssr_blockexpr_replace_trailing_expr_with_stmt () { assert_ssr_transform ("if $a() {$b;} ==>> $b;" , "{
    if foo() {
        bar();
    }
}" , expect ! [["{
    bar();
}"]] ,) ; }
    };
}

ssr_blockexpr_replace_trailing_expr_with_stmt!()