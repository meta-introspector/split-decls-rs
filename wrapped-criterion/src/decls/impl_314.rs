macro_rules! deps {
    () => {
        Distributions!();
        Tuple!();
        Distribution!();
    };
}

macro_rules! impl_314 {
    () => {
        deps!();
        impl < A > Tuple for (A ,) where A : Copy , { type Distributions = (Distribution < A > ,) ; type Builder = (Vec < A > ,) ; }
    };
}

impl_314!()