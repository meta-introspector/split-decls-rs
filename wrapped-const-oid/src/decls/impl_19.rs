macro_rules! deps {
    () => {
        Buffer!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < const SIZE : usize > AsRef < [u8] > for Buffer < SIZE > { fn as_ref (& self) -> & [u8] { self . as_bytes () } }
    };
}

impl_19!()