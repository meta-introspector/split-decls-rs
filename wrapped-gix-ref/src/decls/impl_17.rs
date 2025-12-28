macro_rules! deps {
    () => {
        FullNameRef!();
        FullName!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl ToOwned for FullNameRef { type Owned = FullName ; fn to_owned (& self) -> Self :: Owned { FullName (self . 0 . to_owned ()) } }
    };
}

impl_17!();