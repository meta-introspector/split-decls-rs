macro_rules! deps {
    () => {
        Action!();
        CredentialResponse!();
        RegistryInfo!();
        UnsupportedCredential!();
        Credential!();
        Error!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl Credential for UnsupportedCredential { fn perform (& self , _registry : & RegistryInfo < '_ > , _action : & Action < '_ > , _args : & [& str] ,) -> Result < CredentialResponse , Error > { Err (Error :: UrlNotSupported) } }
    };
}

impl_27!()