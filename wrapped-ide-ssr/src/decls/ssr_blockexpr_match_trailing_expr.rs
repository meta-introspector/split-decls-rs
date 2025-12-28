macro_rules! ssr_blockexpr_match_trailing_expr {
    () => {
        # [test] fn ssr_blockexpr_match_trailing_expr () { assert_matches ("if $a() {$b;}" , "{
    if foo() {
        bar();
    }
}" , & ["if foo() {
        bar();
    }"] ,) ; }
    };
}

ssr_blockexpr_match_trailing_expr!()