// Generated macro for concat_spans (function)
macro_rules! Depcrate_macro_expansion_tests_mbeconcat_spans {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"concat_spans"}
// Dependencies: {}
# [test] fn concat_spans () { check (r#"
#[rustc_builtin_macro]
#[macro_export]
macro_rules! concat {}
macro_rules! identity {
    ($tt:tt) => {
        $tt
    }
}

fn main(foo: ()) {
    #[rustc_builtin_macro]
    #[macro_export]
    macro_rules! concat {}
    macro_rules! identity {
        ($tt:tt) => {
            $tt
        }
    }

    fn main(foo: ()) {
        concat/*+spans+syntaxctxt*/!("hello", concat!("w", identity!("o")), identity!("rld"), unresolved!(), identity!("!"));
    }
}

"# , expect ! [[r##"
#[rustc_builtin_macro]
#[macro_export]
macro_rules! concat {}
macro_rules! identity {
    ($tt:tt) => {
        $tt
    }
}

fn main(foo: ()) {
    #[rustc_builtin_macro]
    #[macro_export]
    macro_rules! concat {}
    macro_rules! identity {
        ($tt:tt) => {
            $tt
        }
    }

    fn main(foo: ()) {
        /* error: unresolved macro unresolved */"helloworld!"#0:Fn[15AE, 0]@236..321#ROOT2024#;
    }
}

"##]] ,) ; }
};
}
