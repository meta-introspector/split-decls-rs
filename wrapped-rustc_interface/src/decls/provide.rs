macro_rules! provide {
    () => {
        pub (crate) fn provide (providers : & mut Providers) { * providers = Providers { proc_macro_decls_static , .. * providers } ; }
    };
}

provide!()