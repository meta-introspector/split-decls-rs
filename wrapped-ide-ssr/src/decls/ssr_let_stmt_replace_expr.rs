macro_rules! ssr_let_stmt_replace_expr {
    () => {
        # [test] fn ssr_let_stmt_replace_expr () { assert_ssr_transform ("let $a = $b; ==>> $b" , "fn main() { let x = 10; }" , expect ! [["fn main() { 10 }"]] ,) ; }
    };
}

ssr_let_stmt_replace_expr!()