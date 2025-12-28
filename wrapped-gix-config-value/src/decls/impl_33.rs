macro_rules! deps {
    () => {
        Integer!();
        Error!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl TryFrom < Cow < '_ , BStr > > for Integer { type Error = Error ; fn try_from (c : Cow < '_ , BStr >) -> Result < Self , Self :: Error > { Self :: try_from (c . as_ref ()) } }
    };
}

impl_33!();