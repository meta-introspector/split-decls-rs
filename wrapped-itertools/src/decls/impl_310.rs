macro_rules! deps {
    () => {
        IteratorIndex!();
    };
}

macro_rules! impl_310 {
    () => {
        deps!();
        impl < I > IteratorIndex < I > for RangeInclusive < usize > where I : Iterator , { type Output = Take < Skip < I > > ; fn index (self , iter : I) -> Self :: Output { let length = if * self . end () == usize :: MAX { assert_ne ! (* self . start () , 0) ; self . end () - self . start () + 1 } else { (self . end () + 1) . saturating_sub (* self . start ()) } ; iter . skip (* self . start ()) . take (length) } }
    };
}

impl_310!();