macro_rules! deps {
    () => {
        CheckStat!();
    };
}

macro_rules! check_stat {
    () => {
        deps!();
        mod check_stat { use std :: borrow :: Cow ; use crate :: { bstr :: { BStr , ByteSlice } , config , config :: tree :: core :: CheckStat , } ; impl CheckStat { # [doc = " Returns true if the full set of stat entries should be checked, and it's just as lenient as git."] pub fn try_into_checkstat (& 'static self , value : Cow < '_ , BStr > ,) -> Result < bool , config :: key :: GenericErrorWithValue > { Ok (match value . as_ref () . as_bytes () { b"minimal" => false , b"default" => true , _ => { return Err (config :: key :: GenericErrorWithValue :: from_value (self , value . into_owned ())) ; } }) } } }
    };
}

check_stat!();