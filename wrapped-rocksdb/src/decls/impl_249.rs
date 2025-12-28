macro_rules! deps {
    () => {
        DBPinnableSlice!();
    };
}

macro_rules! impl_249 {
    () => {
        deps!();
        impl AsRef < [u8] > for DBPinnableSlice < '_ > { fn as_ref (& self) -> & [u8] { self } }
    };
}

impl_249!();