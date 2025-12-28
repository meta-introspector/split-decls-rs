macro_rules! deps {
    () => {
        IteratorIndex!();
    };
}

macro_rules! impl_309 {
    () => {
        deps!();
        impl < I > IteratorIndex < I > for Range < usize > where I : Iterator , { type Output = Skip < Take < I > > ; fn index (self , iter : I) -> Self :: Output { iter . take (self . end) . skip (self . start) } }
    };
}

impl_309!()