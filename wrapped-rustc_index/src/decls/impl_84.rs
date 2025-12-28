macro_rules! deps {
    () => {
        Idx!();
        IntoSliceIdx!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl < I : Idx , T > IntoSliceIdx < I , [T] > for ops :: Range < I > { type Output = ops :: Range < usize > ; # [inline] fn into_slice_idx (self) -> Self :: Output { ops :: Range { start : self . start . index () , end : self . end . index () } } }
    };
}

impl_84!();