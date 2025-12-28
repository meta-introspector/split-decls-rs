macro_rules! deps {
    () => {
        RawValue!();
    };
}

macro_rules! impl_606 {
    () => {
        deps!();
        impl RawValue { const fn from_borrowed (json : & str) -> & Self { unsafe { mem :: transmute :: < & str , & RawValue > (json) } } fn from_owned (json : Box < str >) -> Box < Self > { unsafe { mem :: transmute :: < Box < str > , Box < RawValue > > (json) } } fn into_owned (raw_value : Box < Self >) -> Box < str > { unsafe { mem :: transmute :: < Box < RawValue > , Box < str > > (raw_value) } } }
    };
}

impl_606!();