macro_rules! deps {
    () => {
        BodyRef!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl AsRef < BStr > for BodyRef < '_ > { fn as_ref (& self) -> & BStr { self . body_without_trailer } }
    };
}

impl_9!()