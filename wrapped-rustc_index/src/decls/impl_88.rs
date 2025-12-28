macro_rules! deps {
    () => {
        IntoSliceIdx!();
        Idx!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl < I : Idx , T > IntoSliceIdx < I , [T] > for ops :: RangeToInclusive < I > { type Output = ops :: RangeToInclusive < usize > ; # [inline] fn into_slice_idx (self) -> Self :: Output { ..= self . end . index () } }
    };
}

impl_88!()