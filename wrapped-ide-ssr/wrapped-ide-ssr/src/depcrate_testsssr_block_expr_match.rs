// Generated macro for ssr_block_expr_match (function)
macro_rules! Depcrate_testsssr_block_expr_match {
() => {
// Module: crate::tests
// Provides: {"ssr_block_expr_match"}
// Dependencies: {}
# [test] fn ssr_block_expr_match () { assert_matches ("{ let $a = $b; }" , "fn main() { let x = 10; }" , & ["{ let x = 10; }"]) ; assert_matches ("{ let $a = $b; $c }" , "fn main() { let x = 10; x }" , & ["{ let x = 10; x }"]) ; }
};
}
