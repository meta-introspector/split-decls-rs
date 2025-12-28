macro_rules! deps {
    () => {
        PartialNameRef!();
        Error!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < 'a > convert :: TryFrom < & 'a String > for & 'a PartialNameRef { type Error = Error ; fn try_from (v : & 'a String) -> Result < Self , Self :: Error > { let v = v . as_bytes () . as_bstr () ; Ok (PartialNameRef :: new_unchecked (gix_validate :: reference :: name_partial (v) ?)) } }
    };
}

impl_38!();