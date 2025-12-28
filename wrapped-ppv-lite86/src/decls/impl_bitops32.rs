macro_rules! deps {
    () => {
        RotateEachWord32!();
        BitOps32!();
    };
}

macro_rules! impl_bitops32 {
    () => {
        deps!();
        macro_rules ! impl_bitops32 { ($ vec : ident) => { impl < S3 : Copy , S4 : Copy , NI : Copy > BitOps32 for $ vec < S3 , S4 , NI > where $ vec < S3 , S4 , NI >: RotateEachWord32 { } } ; }
    };
}

impl_bitops32!()