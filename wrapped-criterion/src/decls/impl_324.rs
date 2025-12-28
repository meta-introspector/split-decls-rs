macro_rules! deps {
    () => {
        Distribution!();
        TupledDistributions!();
    };
}

macro_rules! impl_324 {
    () => {
        deps!();
        impl < A , B , C , D > TupledDistributions for (Distribution < A > , Distribution < B > , Distribution < C > , Distribution < D > ,) where A : Copy , B : Copy , C : Copy , D : Copy , { type Item = (A , B , C , D) ; }
    };
}

impl_324!();