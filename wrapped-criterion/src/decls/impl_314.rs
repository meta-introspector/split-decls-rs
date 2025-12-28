macro_rules! deps {
    () => {
        Tuple!();
        Distribution!();
        Distributions!();
    };
}

macro_rules! impl_314 {
    () => {
        deps!();
        impl < A > Tuple for (A ,) where A : Copy , { type Distributions = (Distribution < A > ,) ; type Builder = (Vec < A > ,) ; }
    };
}

impl_314!();