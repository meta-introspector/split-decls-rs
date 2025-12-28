macro_rules! deps {
    () => {
        Action!();
        RegistryInfo!();
        CredentialResponse!();
    };
}

macro_rules! Credential {
    () => {
        deps!();
        pub trait Credential { # [doc = " Retrieves a token for the given registry."] fn perform (& self , registry : & RegistryInfo < '_ > , action : & Action < '_ > , args : & [& str] ,) -> Result < CredentialResponse , Error > ; }
    };
}

Credential!()