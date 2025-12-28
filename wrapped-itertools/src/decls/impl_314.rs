macro_rules! deps {
    () => {
        IteratorIndex!();
    };
}

macro_rules! impl_314 {
    () => {
        deps!();
        impl < I > IteratorIndex < I > for RangeFull where I : Iterator , { type Output = I ; fn index (self , iter : I) -> Self :: Output { iter } }
    };
}

impl_314!();