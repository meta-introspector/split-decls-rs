macro_rules! provide {
    () => {
        pub fn provide (providers : & mut Providers) { * providers = Providers { mir_borrowck , .. * providers } ; }
    };
}

provide!()