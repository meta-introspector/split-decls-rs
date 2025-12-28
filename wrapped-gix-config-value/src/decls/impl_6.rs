macro_rules! deps {
    () => {
        Error!();
        Boolean!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl TryFrom < Cow < '_ , BStr > > for Boolean { type Error = Error ; fn try_from (c : Cow < '_ , BStr >) -> Result < Self , Self :: Error > { Self :: try_from (c . as_ref ()) } }
    };
}

impl_6!()