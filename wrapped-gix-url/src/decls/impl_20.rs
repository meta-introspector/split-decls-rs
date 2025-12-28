macro_rules! deps {
    () => {
        Url!();
        Error!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < 'a > TryFrom < std :: borrow :: Cow < 'a , BStr > > for Url { type Error = parse :: Error ; fn try_from (value : std :: borrow :: Cow < 'a , BStr >) -> Result < Self , Self :: Error > { Self :: try_from (& * value) } }
    };
}

impl_20!();