macro_rules! deps {
    () => {
        BitOps64!();
        RotateEachWord32!();
        RotateEachWord64!();
    };
}

macro_rules! impl_bitops64 {
    () => {
        deps!();
        macro_rules ! impl_bitops64 { ($ vec : ident) => { impl_bitops32 ! ($ vec) ; impl < S3 : Copy , S4 : Copy , NI : Copy > BitOps64 for $ vec < S3 , S4 , NI > where $ vec < S3 , S4 , NI >: RotateEachWord64 + RotateEachWord32 { } } ; }
    };
}

impl_bitops64!();