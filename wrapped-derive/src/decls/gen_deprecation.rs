macro_rules! deps {
    () => {
        Deprecation!();
    };
}

macro_rules! gen_deprecation {
    () => {
        deps!();
        pub fn gen_deprecation (deprecation : & Deprecation , crate_name : & TokenStream) -> TokenStream { match deprecation { Deprecation :: NoDeprecated => { quote ! { # crate_name :: registry :: Deprecation :: NoDeprecated } } Deprecation :: Deprecated { reason : Some (reason) , } => { quote ! { # crate_name :: registry :: Deprecation :: Deprecated { reason : :: std :: option :: Option :: Some (:: std :: string :: ToString :: to_string (# reason)) } } } Deprecation :: Deprecated { reason : None } => { quote ! { # crate_name :: registry :: Deprecation :: Deprecated { reason : :: std :: option :: Option :: None } } } } }
    };
}

gen_deprecation!();