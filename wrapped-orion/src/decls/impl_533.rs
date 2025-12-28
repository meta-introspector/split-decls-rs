macro_rules! deps {
    () => {
        EncapsulationKey!();
    };
}

macro_rules! impl_533 {
    () => {
        deps!();
        impl AsRef < [u8] > for EncapsulationKey { fn as_ref (& self) -> & [u8] { self . value . as_ref () } }
    };
}

impl_533!()