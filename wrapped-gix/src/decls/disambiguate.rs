macro_rules! deps {
    () => {
        Disambiguate!();
        ObjectKindHint!();
        Commit!();
        Tree!();
        Blob!();
    };
}

macro_rules! disambiguate {
    () => {
        deps!();
        # [cfg (feature = "revision")] mod disambiguate { use std :: borrow :: Cow ; use crate :: { bstr :: { BStr , ByteSlice } , config , config :: tree :: core :: Disambiguate , revision :: spec :: parse :: ObjectKindHint , } ; impl Disambiguate { # [doc = " Convert a disambiguation marker into the respective enum."] pub fn try_into_object_kind_hint (& 'static self , value : Cow < '_ , BStr > ,) -> Result < Option < ObjectKindHint > , config :: key :: GenericErrorWithValue > { let hint = match value . as_ref () . as_bytes () { b"none" => return Ok (None) , b"commit" => ObjectKindHint :: Commit , b"committish" => ObjectKindHint :: Committish , b"tree" => ObjectKindHint :: Tree , b"treeish" => ObjectKindHint :: Treeish , b"blob" => ObjectKindHint :: Blob , _ => return Err (config :: key :: GenericErrorWithValue :: from_value (self , value . into_owned ())) , } ; Ok (Some (hint)) } } }
    };
}

disambiguate!()