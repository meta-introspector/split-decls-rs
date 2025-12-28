macro_rules! deps {
    () => {
        PartialNameRef!();
        Error!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < 'a > convert :: TryFrom < & 'a BStr > for & 'a PartialNameRef { type Error = Error ; fn try_from (v : & 'a BStr) -> Result < Self , Self :: Error > { Ok (PartialNameRef :: new_unchecked (gix_validate :: reference :: name_partial (v) ?)) } }
    };
}

impl_31!()