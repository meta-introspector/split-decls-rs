macro_rules! ssr_blockexpr_replace_stmt_with_stmt {
    () => {
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

ssr_blockexpr_replace_stmt_with_stmt!();