macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl TryFrom < Value > for String { type Error = Error ; fn try_from (from : Value) -> Result < Self > { match from . ty { Type :: String | Type :: ExpandString => Ok (Self :: from_utf16 (trim (from . data . as_wide ())) ?) , _ => Err (invalid_data ()) , } } }
    };
}

impl_74!();