macro_rules! deps {
    () => {
        Credential!();
        Boolean!();
        Program!();
        Tree!();
        Any!();
        UrlParameter!();
    };
}

macro_rules! impl_622 {
    () => {
        deps!();
        impl Credential { # [doc = " The `credential.helper` key."] pub const HELPER : keys :: Program = keys :: Program :: new_program ("helper" , & config :: Tree :: CREDENTIAL) ; # [doc = " The `credential.username` key."] pub const USERNAME : keys :: Any = keys :: Any :: new ("username" , & config :: Tree :: CREDENTIAL) ; # [doc = " The `credential.useHttpPath` key."] pub const USE_HTTP_PATH : keys :: Boolean = keys :: Boolean :: new_boolean ("useHttpPath" , & config :: Tree :: CREDENTIAL) ; # [doc = " The `credential.<url>` subsection"] pub const URL_PARAMETER : UrlParameter = UrlParameter ; }
    };
}

impl_622!()