macro_rules! deps {
    () => {
        FullNameRef!();
        Error!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < 'a > convert :: TryFrom < & 'a str > for & 'a FullNameRef { type Error = Error ; fn try_from (v : & 'a str) -> Result < Self , Self :: Error > { let v = v . as_bytes () . as_bstr () ; Ok (FullNameRef :: new_unchecked (gix_validate :: reference :: name (v) ?)) } }
    };
}

impl_33!()