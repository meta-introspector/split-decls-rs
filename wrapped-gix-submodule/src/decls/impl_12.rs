macro_rules! deps {
    () => {
        Update!();
        Error!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl TryFrom < & BStr > for Update { type Error = () ; fn try_from (value : & BStr) -> Result < Self , Self :: Error > { Ok (match value . as_bstr () . as_bytes () { b"checkout" => Update :: Checkout , b"rebase" => Update :: Rebase , b"merge" => Update :: Merge , b"none" => Update :: None , command if command . first () == Some (& b'!') => Update :: Command (command [1 ..] . to_owned () . into ()) , _ => return Err (()) , }) } }
    };
}

impl_12!();