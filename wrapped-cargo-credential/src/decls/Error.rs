macro_rules! deps {
    () => {
        Credential!();
    };
}

macro_rules! Error {
    () => {
        deps!();
        # [doc = " Credential provider error type."] # [doc = ""] # [doc = " `UrlNotSupported` and `NotFound` errors both cause Cargo"] # [doc = " to attempt another provider, if one is available. The other"] # [doc = " variants are fatal."] # [doc = ""] # [doc = " Note: Do not add a tuple variant, as it cannot be serialized."] # [derive (Serialize , Deserialize , ThisError , Debug)] # [serde (rename_all = "kebab-case" , tag = "kind")] # [non_exhaustive] pub enum Error { # [doc = " Registry URL is not supported. This should be used if"] # [doc = " the provider only works for some registries. Cargo will"] # [doc = " try another provider, if available"] # [error ("registry not supported")] UrlNotSupported , # [doc = " Credentials could not be found. Cargo will try another"] # [doc = " provider, if available"] # [error ("credential not found")] NotFound , # [doc = " The provider doesn't support this operation, such as"] # [doc = " a provider that can't support 'login' / 'logout'"] # [error ("requested operation not supported")] OperationNotSupported , # [doc = " The provider failed to perform the operation. Other"] # [doc = " providers will not be attempted"] # [error (transparent)] # [serde (with = "error_serialize")] Other (Box < dyn StdError + Sync + Send >) , # [doc = " A new variant was added to this enum since Cargo was built"] # [error ("unknown error kind; try updating Cargo?")] # [serde (other)] Unknown , }
    };
}

Error!()