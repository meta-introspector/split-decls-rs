// Generated macro for macro_rules_in_attr (function)
macro_rules! Depcrate_macro_expansion_tests_proc_macrosmacro_rules_in_attr {
() => {
// Module: crate::macro_expansion_tests::proc_macros
// Provides: {"macro_rules_in_attr"}
// Dependencies: {}
# [test] fn macro_rules_in_attr () { check (r#"
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
            self.id().await;
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
        self .id().await ;
    }
}
"#]] ,) ; }
};
}
