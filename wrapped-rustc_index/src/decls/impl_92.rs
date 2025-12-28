macro_rules! deps {
    () => {
        Idx!();
        IntoSliceIdx!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        # [cfg (all (feature = "nightly" , not (bootstrap)))] impl < I : Idx , T > IntoSliceIdx < I , [T] > for core :: range :: RangeToInclusive < I > { type Output = core :: range :: RangeToInclusive < usize > ; # [inline] fn into_slice_idx (self) -> Self :: Output { core :: range :: RangeToInclusive { last : self . last . index () } } }
    };
}

impl_92!();