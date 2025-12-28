macro_rules! deps {
    () => {
        PartialName!();
        Error!();
        PartialNameRef!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < 'a > convert :: TryFrom < & 'a PartialName > for & 'a PartialNameRef { type Error = Error ; fn try_from (v : & 'a PartialName) -> Result < Self , Self :: Error > { Ok (PartialNameRef :: new_unchecked (v . 0 . as_bstr ())) } }
    };
}

impl_32!()