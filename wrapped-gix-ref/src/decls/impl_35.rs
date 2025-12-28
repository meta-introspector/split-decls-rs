macro_rules! deps {
    () => {
        PartialName!();
        Error!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < 'a > convert :: TryFrom < & 'a str > for PartialName { type Error = Error ; fn try_from (v : & 'a str) -> Result < Self , Self :: Error > { let v = v . as_bytes () . as_bstr () ; Ok (PartialName (gix_validate :: reference :: name_partial (v) ? . to_owned ())) } }
    };
}

impl_35!();