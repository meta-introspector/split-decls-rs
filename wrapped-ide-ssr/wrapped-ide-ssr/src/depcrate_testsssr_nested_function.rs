// Generated macro for ssr_nested_function (function)
macro_rules! Depcrate_testsssr_nested_function {
() => {
// Module: crate::tests
// Provides: {"ssr_nested_function"}
// Dependencies: {}
# [test] fn ssr_nested_function () { assert_ssr_transform ("foo($a, $b, $c) ==>> bar($c, baz($a, $b))" , r#"
            //- /lib.rs crate:foo
            fn foo() {}
            fn bar() {}
            fn baz() {}
            fn main { foo  (x + value.method(b), x+y-z, true && false) }
            "# , expect ! [[r#"
            fn foo() {}
            fn bar() {}
            fn baz() {}
            fn main { bar(true && false, baz(x + value.method(b), x+y-z)) }
        "#]] ,) }
};
}
