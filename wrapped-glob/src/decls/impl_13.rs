macro_rules! deps {
    () => {
        PathWrapper!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl AsRef < Path > for PathWrapper { fn as_ref (& self) -> & Path { self . path . as_ref () } }
    };
}

impl_13!();