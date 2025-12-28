macro_rules! deps {
    () => {
        FullName!();
        Error!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl TryFrom < & BString > for FullName { type Error = gix_validate :: reference :: name :: Error ; fn try_from (value : & BString) -> Result < Self , Self :: Error > { gix_validate :: reference :: name (value . as_ref ()) ? ; Ok (FullName (value . clone ())) } }
    };
}

impl_5!()