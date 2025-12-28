macro_rules! macro_rules_in_attr {
    () => {
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

macro_rules_in_attr!()