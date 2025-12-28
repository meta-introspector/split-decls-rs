macro_rules! deps {
    () => {
        Idx!();
        IntoSliceIdx!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl < I : Idx , T > IntoSliceIdx < I , [T] > for ops :: RangeTo < I > { type Output = ops :: RangeTo < usize > ; # [inline] fn into_slice_idx (self) -> Self :: Output { .. self . end . index () } }
    };
}

impl_86!();