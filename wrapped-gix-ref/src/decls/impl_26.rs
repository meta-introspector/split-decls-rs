macro_rules! deps {
    () => {
        FullNameRef!();
        Error!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < 'a > convert :: TryFrom < & 'a BStr > for & 'a FullNameRef { type Error = Error ; fn try_from (v : & 'a BStr) -> Result < Self , Self :: Error > { Ok (FullNameRef :: new_unchecked (gix_validate :: reference :: name (v) ?)) } }
    };
}

impl_26!();