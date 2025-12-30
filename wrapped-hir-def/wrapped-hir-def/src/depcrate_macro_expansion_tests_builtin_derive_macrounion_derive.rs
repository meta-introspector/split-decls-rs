// Generated macro for union_derive (function)
macro_rules! Depcrate_macro_expansion_tests_builtin_derive_macrounion_derive {
() => {
// Module: crate::macro_expansion_tests::builtin_derive_macro
// Provides: {"union_derive"}
// Dependencies: {}
# [test] fn union_derive () { check_errors (r#"
//- minicore: clone, copy, default, fmt, hash, ord, eq, derive

#[derive(Copy)]
union Foo1 { _v: () }
#[derive(Clone)]
union Foo2 { _v: () }
#[derive(Default)]
union Foo3 { _v: () }
#[derive(Debug)]
union Foo4 { _v: () }
#[derive(Hash)]
union Foo5 { _v: () }
#[derive(Ord)]
union Foo6 { _v: () }
#[derive(PartialOrd)]
union Foo7 { _v: () }
#[derive(Eq)]
union Foo8 { _v: () }
#[derive(PartialEq)]
union Foo9 { _v: () }
    "# , expect ! [[r#"
            78..118: this trait cannot be derived for unions
            119..157: this trait cannot be derived for unions
            158..195: this trait cannot be derived for unions
            196..232: this trait cannot be derived for unions
            233..276: this trait cannot be derived for unions
            313..355: this trait cannot be derived for unions"#]] ,) ; }
};
}
