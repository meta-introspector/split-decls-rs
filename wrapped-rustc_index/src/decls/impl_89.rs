macro_rules! deps {
    () => {
        IntoSliceIdx!();
        Idx!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        # [cfg (feature = "nightly")] impl < I : Idx , T > IntoSliceIdx < I , [T] > for core :: range :: Range < I > { type Output = core :: range :: Range < usize > ; # [inline] fn into_slice_idx (self) -> Self :: Output { core :: range :: Range { start : self . start . index () , end : self . end . index () } } }
    };
}

impl_89!();