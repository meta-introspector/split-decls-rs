macro_rules! extern_crate {
    () => {
        pub (crate) mod extern_crate ;
    };
}

extern_crate!()