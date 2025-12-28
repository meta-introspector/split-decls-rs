macro_rules! replace_associated_trait_constant {
    () => {
        # [test] fn replace_associated_trait_constant () { cov_mark :: check ! (replace_associated_trait_constant) ; assert_ssr_transform ("Bar2::VALUE ==>> Bar2::VALUE_2222" , r#"
            trait Foo { const VALUE: i32; const VALUE_2222: i32; }
            pub(crate) struct Bar {}
            impl Foo for Bar { const VALUE: i32 = 1;  const VALUE_2222: i32 = 2; }
            pub(crate) struct Bar2 {}
            impl Foo for Bar2 { const VALUE: i32 = 1;  const VALUE_2222: i32 = 2; }
            impl Bar2 { fn foo2() {} }
            fn main() {
                Bar::VALUE;
                Bar2::VALUE;
            }
            "# , expect ! [[r#"
            trait Foo { const VALUE: i32; const VALUE_2222: i32; }
            pub(crate) struct Bar {}
            impl Foo for Bar { const VALUE: i32 = 1;  const VALUE_2222: i32 = 2; }
            pub(crate) struct Bar2 {}
            impl Foo for Bar2 { const VALUE: i32 = 1;  const VALUE_2222: i32 = 2; }
            impl Bar2 { fn foo2() {} }
            fn main() {
                Bar::VALUE;
                Bar2::VALUE_2222;
            }
        "#]] ,) ; }
    };
}

replace_associated_trait_constant!();