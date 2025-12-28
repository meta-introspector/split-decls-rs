macro_rules! deps {
    () => {
        Merge!();
        ConflictStyle!();
    };
}

macro_rules! conflict_style {
    () => {
        deps!();
        # [cfg (feature = "merge")] mod conflict_style { use std :: borrow :: Cow ; use gix_merge :: blob :: builtin_driver :: text ; use crate :: { bstr :: BStr , config , config :: tree :: sections :: merge :: ConflictStyle } ; impl ConflictStyle { # [doc = " Derive the diff algorithm identified by `name`, case-insensitively."] pub fn try_into_conflict_style (& 'static self , name : Cow < '_ , BStr > ,) -> Result < text :: ConflictStyle , config :: key :: GenericErrorWithValue > { let style = if name . as_ref () == "merge" { text :: ConflictStyle :: Merge } else if name . as_ref () == "diff3" { text :: ConflictStyle :: Diff3 } else if name . as_ref () == "zdiff3" { text :: ConflictStyle :: ZealousDiff3 } else { return Err (config :: key :: GenericErrorWithValue :: from_value (self , name . into_owned ())) ; } ; Ok (style) } } }
    };
}

conflict_style!();