macro_rules! deps {
    () => {
        Error!();
        FullName!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl TryFrom < & BStr > for FullName { type Error = gix_validate :: reference :: name :: Error ; fn try_from (value : & BStr) -> Result < Self , Self :: Error > { Ok (FullName (gix_validate :: reference :: name (value) ? . into ())) } }
    };
}

impl_3!();