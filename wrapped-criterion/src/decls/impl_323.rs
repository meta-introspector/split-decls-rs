macro_rules! deps {
    () => {
        Distribution!();
        Tuple!();
        Distributions!();
    };
}

macro_rules! impl_323 {
    () => {
        deps!();
        impl < A , B , C , D > Tuple for (A , B , C , D) where A : Copy , B : Copy , C : Copy , D : Copy , { type Distributions = (Distribution < A > , Distribution < B > , Distribution < C > , Distribution < D > ,) ; type Builder = (Vec < A > , Vec < B > , Vec < C > , Vec < D >) ; }
    };
}

impl_323!()