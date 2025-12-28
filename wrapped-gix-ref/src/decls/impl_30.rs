macro_rules! deps {
    () => {
        PartialNameRef!();
        Error!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < 'a > convert :: TryFrom < & 'a BString > for & 'a PartialNameRef { type Error = Error ; fn try_from (v : & 'a BString) -> Result < Self , Self :: Error > { Ok (PartialNameRef :: new_unchecked (gix_validate :: reference :: name_partial (v . as_ref () ,) ?)) } }
    };
}

impl_30!();