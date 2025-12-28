macro_rules! IdentityRef {
    () => {
        # [doc = " A person with name and email, as reference."] # [derive (Default , PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct IdentityRef < 'a > { # [doc = " The actors name, potentially with whitespace as parsed."] # [doc = ""] # [doc = " Use [IdentityRef::trim()] or trim manually to be able to clean it up."] # [cfg_attr (feature = "serde" , serde (borrow))] pub name : & 'a BStr , # [doc = " The actor's email, potentially with whitespace and garbage as parsed."] # [doc = ""] # [doc = " Use [IdentityRef::trim()] or trim manually to be able to clean it up."] pub email : & 'a BStr , }
    };
}

IdentityRef!();