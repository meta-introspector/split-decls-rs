macro_rules! Identity {
    () => {
        # [doc = " A person with name and email."] # [derive (Default , PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Identity { # [doc = " The actors name, potentially with whitespace as parsed."] # [doc = ""] # [doc = " Use [IdentityRef::trim()] or trim manually to be able to clean it up."] pub name : BString , # [doc = " The actor's email, potentially with whitespace and garbage as parsed."] # [doc = ""] # [doc = " Use [IdentityRef::trim()] or trim manually to be able to clean it up."] pub email : BString , }
    };
}

Identity!();