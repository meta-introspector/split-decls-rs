macro_rules! Signature {
    () => {
        # [doc = " A signature like [`gix_actor::Signature`], but with all string fields being a `Cow`."] # [derive (Default , PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Signature < 'a > { # [doc = " The possibly mapped name."] pub name : Cow < 'a , BStr > , # [doc = " The possibly mapped email."] pub email : Cow < 'a , BStr > , # [doc = " The time stamp at which the signature is performed."] pub time : gix_date :: Time , }
    };
}

Signature!()