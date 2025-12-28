macro_rules! deps {
    () => {
        BitOps128!();
        RotateEachWord128!();
    };
}

macro_rules! impl_bitops128 {
    () => {
        deps!();
        macro_rules ! impl_bitops128 { ($ vec : ident) => { impl_bitops64 ! ($ vec) ; impl < S3 : Copy , S4 : Copy , NI : Copy > BitOps128 for $ vec < S3 , S4 , NI > where $ vec < S3 , S4 , NI >: RotateEachWord128 { } } ; }
    };
}

impl_bitops128!()