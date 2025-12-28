macro_rules! deps {
    () => {
        CredentialResponse!();
        Credential!();
        RegistryInfo!();
        UnsupportedCredential!();
        Action!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl Credential for UnsupportedCredential { fn perform (& self , _registry : & RegistryInfo < '_ > , _action : & Action < '_ > , _args : & [& str] ,) -> Result < CredentialResponse , Error > { Err (Error :: UrlNotSupported) } }
    };
}

impl_5!()