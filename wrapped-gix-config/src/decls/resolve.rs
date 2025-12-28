macro_rules! deps {
    () => {
        File!();
        Options!();
        Error!();
    };
}

macro_rules! resolve {
    () => {
        deps!();
        pub (crate) fn resolve (config : & mut File < 'static > , buf : & mut Vec < u8 > , options : init :: Options < '_ >) -> Result < () , Error > { resolve_includes_recursive (None , config , 0 , buf , options) }
    };
}

resolve!();