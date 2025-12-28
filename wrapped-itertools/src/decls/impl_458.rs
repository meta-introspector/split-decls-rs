macro_rules! deps {
    () => {
        RcIter!();
    };
}

macro_rules! impl_458 {
    () => {
        deps!();
        impl < I > Clone for RcIter < I > { clone_fields ! (rciter) ; }
    };
}

impl_458!();