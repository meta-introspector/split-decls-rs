macro_rules! deps {
    () => {
        Idx!();
        IntoSliceIdx!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl < I : Idx , T > IntoSliceIdx < I , [T] > for I { type Output = usize ; # [inline] fn into_slice_idx (self) -> Self :: Output { self . index () } }
    };
}

impl_82!()