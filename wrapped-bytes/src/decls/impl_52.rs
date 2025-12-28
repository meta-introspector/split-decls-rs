macro_rules! deps {
    () => {
        UninitSlice!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl < 'a > From < & 'a mut [u8] > for & 'a mut UninitSlice { fn from (slice : & 'a mut [u8]) -> Self { UninitSlice :: new (slice) } }
    };
}

impl_52!()