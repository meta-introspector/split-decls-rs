macro_rules! deps {
    () => {
        IteratorIndex!();
    };
}

macro_rules! impl_312 {
    () => {
        deps!();
        impl < I > IteratorIndex < I > for RangeToInclusive < usize > where I : Iterator , { type Output = Take < I > ; fn index (self , iter : I) -> Self :: Output { assert_ne ! (self . end , usize :: MAX) ; iter . take (self . end + 1) } }
    };
}

impl_312!()