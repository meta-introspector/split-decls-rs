macro_rules! ssr_block_expr_match {
    () => {
        # [test] fn ssr_block_expr_match () { assert_matches ("{ let $a = $b; }" , "fn main() { let x = 10; }" , & ["{ let x = 10; }"]) ; assert_matches ("{ let $a = $b; $c }" , "fn main() { let x = 10; x }" , & ["{ let x = 10; x }"]) ; }
    };
}

ssr_block_expr_match!();