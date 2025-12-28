macro_rules! deps {
    () => {
        Error!();
        PartialName!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl convert :: TryFrom < String > for PartialName { type Error = Error ; fn try_from (v : String) -> Result < Self , Self :: Error > { gix_validate :: reference :: name_partial (v . as_bytes () . as_bstr ()) ? ; Ok (PartialName (v . into ())) } }
    };
}

impl_39!()