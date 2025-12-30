// Generated macro for ssr_blockexpr_replace_stmt_with_stmt (function)
macro_rules! Depcrate_testsssr_blockexpr_replace_stmt_with_stmt {
() => {
// Module: crate::tests
// Provides: {"ssr_blockexpr_replace_stmt_with_stmt"}
// Dependencies: {}
# [test] fn ssr_blockexpr_replace_stmt_with_stmt () { assert_ssr_transform ("if $a() {$b;} ==>> $b;" , "{
    if foo() {
        bar();
    }
    Ok(())
}" , expect ! [[r#"{
    bar();
    Ok(())
}"#]] ,) ; }
};
}
