macro_rules! deps {
    () => {
        Error!();
        PartialName!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl convert :: TryFrom < BString > for PartialName { type Error = Error ; fn try_from (v : BString) -> Result < Self , Self :: Error > { gix_validate :: reference :: name_partial (v . as_ref ()) ? ; Ok (PartialName (v)) } }
    };
}

impl_40!();