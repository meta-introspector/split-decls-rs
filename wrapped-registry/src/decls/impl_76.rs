macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl TryFrom < Value > for Vec < String > { type Error = Error ; fn try_from (from : Value) -> Result < Self > { match from . ty { Type :: MultiString => Ok (from . data . as_wide () . split (| c | * c == 0) . map (String :: from_utf16_lossy) . collect ()) , _ => Ok (vec ! [String :: try_from (from) ?]) , } } }
    };
}

impl_76!()