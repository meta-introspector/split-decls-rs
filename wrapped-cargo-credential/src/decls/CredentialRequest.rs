macro_rules! deps {
    () => {
        RegistryInfo!();
        Action!();
    };
}

macro_rules! CredentialRequest {
    () => {
        deps!();
        # [doc = " Message sent by Cargo to the credential helper after the hello"] # [derive (Serialize , Deserialize , Clone , Debug , PartialEq , Eq)] # [serde (rename_all = "kebab-case")] pub struct CredentialRequest < 'a > { pub v : u32 , # [serde (borrow)] pub registry : RegistryInfo < 'a > , # [serde (borrow , flatten)] pub action : Action < 'a > , # [doc = " Additional command-line arguments passed to the credential provider."] # [serde (skip_serializing_if = "Vec::is_empty" , default)] pub args : Vec < & 'a str > , }
    };
}

CredentialRequest!();