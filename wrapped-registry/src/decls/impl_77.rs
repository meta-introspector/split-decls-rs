macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl TryFrom < Value > for HSTRING { type Error = Error ; fn try_from (from : Value) -> Result < Self > { match from . ty { Type :: String | Type :: ExpandString => Ok (Self :: from_wide (trim (from . data . as_wide ()))) , _ => Err (invalid_data ()) , } } }
    };
}

impl_77!();