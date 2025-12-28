macro_rules! deps {
    () => {
        IntoSliceIdx!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl < I , T > IntoSliceIdx < I , [T] > for ops :: RangeFull { type Output = ops :: RangeFull ; # [inline] fn into_slice_idx (self) -> Self :: Output { self } }
    };
}

impl_83!();