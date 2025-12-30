// Generated macro for test_punct_without_space (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_punct_without_space {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_punct_without_space"}
// Dependencies: {}
# [test] fn test_punct_without_space () { check (r#"
macro_rules! foo {
    (: : :) => { "1 1 1" };
    (: ::) => { "1 2" };
    (:: :) => { "2 1" };

    (: : : :) => { "1 1 1 1" };
    (:: : :) => { "2 1 1" };
    (: :: :) => { "1 2 1" };
    (: : ::) => { "1 1 2" };
    (:: ::) => { "2 2" };
}

fn test() {
    foo!(:::);
    foo!(: :::);
    foo!(::::);
}
"# , expect ! [[r#"
macro_rules! foo {
    (: : :) => { "1 1 1" };
    (: ::) => { "1 2" };
    (:: :) => { "2 1" };

    (: : : :) => { "1 1 1 1" };
    (:: : :) => { "2 1 1" };
    (: :: :) => { "1 2 1" };
    (: : ::) => { "1 1 2" };
    (:: ::) => { "2 2" };
}

fn test() {
    "2 1";
    "1 2 1";
    "2 2";
}
"#]] ,) ; }
};
}
