macro_rules! deps {
    () => {
        Powerset!();
    };
}

macro_rules! impl_437 {
    () => {
        deps!();
        impl < I > Clone for Powerset < I > where I : Clone + Iterator , I :: Item : Clone , { clone_fields ! (combs) ; }
    };
}

impl_437!();