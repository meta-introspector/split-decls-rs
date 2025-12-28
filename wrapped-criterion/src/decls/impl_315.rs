macro_rules! deps {
    () => {
        Distribution!();
        TupledDistributions!();
    };
}

macro_rules! impl_315 {
    () => {
        deps!();
        impl < A > TupledDistributions for (Distribution < A > ,) where A : Copy , { type Item = (A ,) ; }
    };
}

impl_315!();