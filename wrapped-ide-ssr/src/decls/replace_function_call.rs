macro_rules! replace_function_call {
    () => {
        # [test] fn replace_function_call () { assert_ssr_transform ("foo() ==>> bar()" , "fn foo() {$0$0} fn bar() {} fn f1() {foo(); foo();}" , expect ! [["fn foo() {} fn bar() {} fn f1() {bar(); bar();}"]] ,) ; }
    };
}

replace_function_call!();