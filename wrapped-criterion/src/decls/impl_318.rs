macro_rules! deps {
    () => {
        TupledDistributions!();
        Distribution!();
    };
}

macro_rules! impl_318 {
    () => {
        deps!();
        impl < A , B > TupledDistributions for (Distribution < A > , Distribution < B >) where A : Copy , B : Copy , { type Item = (A , B) ; }
    };
}

impl_318!()