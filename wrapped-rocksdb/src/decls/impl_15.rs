macro_rules! deps {
    () => {
        CSlice!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl AsRef < [u8] > for CSlice { fn as_ref (& self) -> & [u8] { unsafe { std :: slice :: from_raw_parts (self . data as * const u8 , self . len) } } }
    };
}

impl_15!();