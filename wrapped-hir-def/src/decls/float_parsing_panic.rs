macro_rules! float_parsing_panic {
    () => {
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

float_parsing_panic!();