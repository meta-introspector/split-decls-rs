macro_rules! deps {
    () => {
        FullName!();
        Error!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl TryFrom < String > for FullName { type Error = gix_validate :: reference :: name :: Error ; fn try_from (value : String) -> Result < Self , Self :: Error > { gix_validate :: reference :: name (value . as_bytes () . as_bstr ()) ? ; Ok (FullName (value . into ())) } }
    };
}

impl_2!();