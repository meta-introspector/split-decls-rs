macro_rules! deps {
    () => {
        IntoSliceIdx!();
        Idx!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl < I : Idx , T > IntoSliceIdx < I , [T] > for ops :: RangeFrom < I > { type Output = ops :: RangeFrom < usize > ; # [inline] fn into_slice_idx (self) -> Self :: Output { ops :: RangeFrom { start : self . start . index () } } }
    };
}

impl_85!()