macro_rules! deps {
    () => {
        Url!();
        Error!();
        Options!();
        Action!();
        Snapshot!();
    };
}

macro_rules! impl_572 {
    () => {
        deps!();
        impl Snapshot < '_ > { # [doc = " Returns the configuration for all git-credential helpers from trusted configuration that apply"] # [doc = " to the given `url` along with an action preconfigured to invoke the cascade with."] # [doc = " For details, please see [this function](function::credential_helpers)."] pub fn credential_helpers (& self , url : gix_url :: Url ,) -> Result < (gix_credentials :: helper :: Cascade , gix_credentials :: helper :: Action , gix_prompt :: Options < 'static > ,) , Error , > { let repo = self . repo ; function :: credential_helpers (url , & repo . config . resolved , repo . config . lenient_config , & mut repo . filter_config_section () , repo . config . environment , false ,) } }
    };
}

impl_572!();