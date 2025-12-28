macro_rules! deps {
    () => {
        ObjectFormat!();
        Kind!();
    };
}

macro_rules! object_format {
    () => {
        deps!();
        mod object_format { use std :: borrow :: Cow ; use crate :: { bstr :: BStr , config , config :: tree :: sections :: extensions :: ObjectFormat } ; impl ObjectFormat { pub fn try_into_object_format (& 'static self , value : Cow < '_ , BStr > ,) -> Result < gix_hash :: Kind , config :: key :: GenericErrorWithValue > { if value . as_ref () . eq_ignore_ascii_case (b"sha1") { Ok (gix_hash :: Kind :: Sha1) } else { Err (config :: key :: GenericErrorWithValue :: from_value (self , value . into_owned ())) } } } }
    };
}

object_format!()