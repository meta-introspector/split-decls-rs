macro_rules! no_implicit_prelude {
    () => {
        pub (crate) mod no_implicit_prelude ;
    };
}

no_implicit_prelude!();