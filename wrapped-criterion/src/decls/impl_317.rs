macro_rules! deps {
    () => {
        Distributions!();
        Distribution!();
        Tuple!();
    };
}

macro_rules! impl_317 {
    () => {
        deps!();
        impl < A , B > Tuple for (A , B) where A : Copy , B : Copy , { type Distributions = (Distribution < A > , Distribution < B >) ; type Builder = (Vec < A > , Vec < B >) ; }
    };
}

impl_317!()