macro_rules! deps {
    () => {
        TupledDistributions!();
        Distribution!();
    };
}

macro_rules! impl_321 {
    () => {
        deps!();
        impl < A , B , C > TupledDistributions for (Distribution < A > , Distribution < B > , Distribution < C >) where A : Copy , B : Copy , C : Copy , { type Item = (A , B , C) ; }
    };
}

impl_321!();