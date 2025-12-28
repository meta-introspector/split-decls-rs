macro_rules! replace_nested_function_calls {
    () => {
        # [test] fn replace_nested_function_calls () { assert_ssr_transform ("foo($a) ==>> bar($a)" , "fn foo() {} fn bar() {} fn f1() {foo(foo(42))}" , expect ! [["fn foo() {} fn bar() {} fn f1() {bar(bar(42))}"]] ,) ; }
    };
}

replace_nested_function_calls!()