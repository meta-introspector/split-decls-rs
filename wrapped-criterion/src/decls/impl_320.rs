macro_rules! deps {
    () => {
        Distributions!();
        Tuple!();
        Distribution!();
    };
}

macro_rules! impl_320 {
    () => {
        deps!();
        impl < A , B , C > Tuple for (A , B , C) where A : Copy , B : Copy , C : Copy , { type Distributions = (Distribution < A > , Distribution < B > , Distribution < C >) ; type Builder = (Vec < A > , Vec < B > , Vec < C >) ; }
    };
}

impl_320!();