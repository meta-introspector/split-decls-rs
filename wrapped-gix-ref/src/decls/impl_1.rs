macro_rules! deps {
    () => {
        Error!();
        FullName!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl TryFrom < & str > for FullName { type Error = gix_validate :: reference :: name :: Error ; fn try_from (value : & str) -> Result < Self , Self :: Error > { Ok (FullName (gix_validate :: reference :: name (value . as_bytes () . as_bstr ()) ? . into () ,)) } }
    };
}

impl_1!();