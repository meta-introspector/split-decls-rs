macro_rules! ssr_let_stmt_in_fn_match {
    () => {
        # [test] fn ssr_let_stmt_in_fn_match () { assert_matches ("let $a = 10;" , "fn main() { let x = 10; x }" , & ["let x = 10;"]) ; assert_matches ("let $a = $b;" , "fn main() { let x = 10; x }" , & ["let x = 10;"]) ; }
    };
}

ssr_let_stmt_in_fn_match!();