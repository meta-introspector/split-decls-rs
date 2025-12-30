// Generated macro for test_last_expr (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_last_expr {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_last_expr"}
// Dependencies: {}
# [test] fn test_last_expr () { check (r#"
macro_rules! vec {
    ($($item:expr),*) => {{
            let mut v = Vec::new();
            $( v.push($item); )*
            v
    }};
}

fn f() {
    vec![1,2,3];
}
"# , expect ! [[r#"
macro_rules! vec {
    ($($item:expr),*) => {{
            let mut v = Vec::new();
            $( v.push($item); )*
            v
    }};
}

fn f() {
     {
        let mut v = Vec::new();
        v.push(1);
        v.push(2);
        v.push(3);
        v
    };
}
"#]] ,) ; }
};
}
