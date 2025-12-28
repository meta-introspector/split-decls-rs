macro_rules! ssr_let_stmt_replace {
    () => {
        # [test] fn ssr_let_stmt_replace () { assert_ssr_transform ("let $a = $b; ==>> let $a = 11;" , "fn main() { let x = 10; x }" , expect ! [["fn main() { let x = 11; x }"]] ,) ; }
    };
}

ssr_let_stmt_replace!()