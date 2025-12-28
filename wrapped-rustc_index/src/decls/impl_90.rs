macro_rules! deps {
    () => {
        IntoSliceIdx!();
        Idx!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        # [cfg (feature = "nightly")] impl < I : Idx , T > IntoSliceIdx < I , [T] > for core :: range :: RangeFrom < I > { type Output = core :: range :: RangeFrom < usize > ; # [inline] fn into_slice_idx (self) -> Self :: Output { core :: range :: RangeFrom { start : self . start . index () } } }
    };
}

impl_90!();