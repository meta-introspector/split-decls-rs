macro_rules! deps {
    () => {
        IteratorIndex!();
    };
}

macro_rules! impl_311 {
    () => {
        deps!();
        impl < I > IteratorIndex < I > for RangeTo < usize > where I : Iterator , { type Output = Take < I > ; fn index (self , iter : I) -> Self :: Output { iter . take (self . end) } }
    };
}

impl_311!()