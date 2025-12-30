// Generated macro for replace_local_variable_reference (function)
macro_rules! Depcrate_testsreplace_local_variable_reference {
() => {
// Module: crate::tests
// Provides: {"replace_local_variable_reference"}
// Dependencies: {}
# [test] fn replace_local_variable_reference () { cov_mark :: check ! (cursor_after_semicolon) ; assert_ssr_transform ("foo + $a ==>> $a - foo" , r#"
            fn bar1() -> i32 {
                let mut res = 0;
                let foo = 5;
                res += foo + 1;
                let foo = 10;
                res += foo + 2;$0
                res += foo + 3;
                let foo = 15;
                res += foo + 4;
                res
            }
            "# , expect ! [[r#"
            fn bar1() -> i32 {
                let mut res = 0;
                let foo = 5;
                res += foo + 1;
                let foo = 10;
                res += 2 - foo;
                res += 3 - foo;
                let foo = 15;
                res += foo + 4;
                res
            }
        "#]] ,) }
};
}
