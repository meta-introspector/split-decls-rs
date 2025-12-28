macro_rules! deps {
    () => {
        Note!();
        TagOpt!();
    };
}

macro_rules! tag_opts {
    () => {
        deps!();
        mod tag_opts { use std :: borrow :: Cow ; use crate :: { bstr :: { BStr , ByteSlice } , config , config :: tree :: remote :: TagOpt , remote , } ; impl TagOpt { # [doc = " Try to interpret `value` as tag option."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " It's heavily biased towards the git command-line unfortunately, and the only"] # [doc = " value of its kind. Maybe in future more values will be supported which are less"] # [doc = " about passing them to a sub-process."] pub fn try_into_tag_opt (& 'static self , value : Cow < '_ , BStr > ,) -> Result < remote :: fetch :: Tags , config :: key :: GenericErrorWithValue > { Ok (match value . as_ref () . as_bytes () { b"--tags" => remote :: fetch :: Tags :: All , b"--no-tags" => remote :: fetch :: Tags :: None , _ => return Err (config :: key :: GenericErrorWithValue :: from_value (self , value . into_owned ())) , }) } } }
    };
}

tag_opts!()