macro_rules! deps {
    () => {
        RawValue!();
    };
}

macro_rules! impl_608 {
    () => {
        deps!();
        impl ToOwned for RawValue { type Owned = Box < RawValue > ; fn to_owned (& self) -> Self :: Owned { RawValue :: from_owned (self . json . to_owned () . into_boxed_str ()) } }
    };
}

impl_608!();