macro_rules! ThorinErrorWrapper {
    () => {
        pub (crate) struct ThorinErrorWrapper (pub thorin :: Error) ;
    };
}

ThorinErrorWrapper!()