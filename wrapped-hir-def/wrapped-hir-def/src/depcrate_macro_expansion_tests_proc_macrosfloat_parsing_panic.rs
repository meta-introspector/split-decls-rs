// Generated macro for float_parsing_panic (function)
macro_rules! Depcrate_macro_expansion_tests_proc_macrosfloat_parsing_panic {
() => {
// Module: crate::macro_expansion_tests::proc_macros
// Provides: {"float_parsing_panic"}
// Dependencies: {}
# [test] fn float_parsing_panic () { check (r#"
//- proc_macros: identity
macro_rules! id {
    ($($t:tt)*) => {
        $($t)*
    };
}
id! {
    #[proc_macros::identity]
    impl Foo for WrapBj {
        async fn foo(&self) {
            self.0. id().await;
        }
    }
}
"# , expect ! [[r#"
macro_rules! id {
    ($($t:tt)*) => {
        $($t)*
    };
}
#[proc_macros::identity] impl Foo for WrapBj {
    async fn foo(&self ) {
        self .0.id().await ;
    }
}
"#]] ,) ; }
};
}
