macro_rules! deps {
    () => {
        Color!();
        Error!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl TryFrom < Cow < '_ , BStr > > for Color { type Error = Error ; fn try_from (c : Cow < '_ , BStr >) -> Result < Self , Self :: Error > { Self :: try_from (c . as_ref ()) } }
    };
}

impl_16!();