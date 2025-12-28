macro_rules! deps {
    () => {
        UninitSlice!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < 'a > From < & 'a mut [MaybeUninit < u8 >] > for & 'a mut UninitSlice { fn from (slice : & 'a mut [MaybeUninit < u8 >]) -> Self { UninitSlice :: uninit (slice) } }
    };
}

impl_53!();