macro_rules! deps {
    () => {
        IteratorIndex!();
    };
}

macro_rules! impl_313 {
    () => {
        deps!();
        impl < I > IteratorIndex < I > for RangeFrom < usize > where I : Iterator , { type Output = Skip < I > ; fn index (self , iter : I) -> Self :: Output { iter . skip (self . start) } }
    };
}

impl_313!()