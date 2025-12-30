// Generated macro for test_inner_macro_rules (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_inner_macro_rules {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_inner_macro_rules"}
// Dependencies: {}
# [test] fn test_inner_macro_rules () { check (r#"
macro_rules! m {
    ($a:ident, $b:ident, $c:tt) => {
        macro_rules! inner {
            ($bi:ident) => { fn $bi() -> u8 { $c } }
        }

        inner!($a);
        fn $b() -> u8 { $c }
    }
}
m!(x, y, 1);
"# , expect ! [[r#"
macro_rules! m {
    ($a:ident, $b:ident, $c:tt) => {
        macro_rules! inner {
            ($bi:ident) => { fn $bi() -> u8 { $c } }
        }

        inner!($a);
        fn $b() -> u8 { $c }
    }
}
macro_rules !inner {
    ($bi: ident) = > {
        fn $bi()-> u8 {
            1
        }
    }
}
inner!(x);
fn y() -> u8 {
    1
}
"#]] ,) ; }
};
}
