macro_rules! deps {
    () => {
        Default!();
    };
}

macro_rules! default {
    () => {
        deps!();
        mod default { use std :: borrow :: Cow ; use crate :: { bstr :: { BStr , ByteSlice } , config , config :: tree :: push :: Default , push , } ; impl Default { # [doc = " Try to interpret `value` as `push.default`."] pub fn try_into_default (& 'static self , value : Cow < '_ , BStr > ,) -> Result < push :: Default , config :: key :: GenericErrorWithValue > { Ok (match value . as_ref () . as_bytes () { b"nothing" => push :: Default :: Nothing , b"current" => push :: Default :: Current , b"upstream" | b"tracking" => push :: Default :: Upstream , b"simple" => push :: Default :: Simple , b"matching" => push :: Default :: Matching , _ => return Err (config :: key :: GenericErrorWithValue :: from_value (self , value . into_owned ())) , }) } } }
    };
}

default!()