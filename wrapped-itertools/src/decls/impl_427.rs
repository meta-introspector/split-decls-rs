macro_rules! deps {
    () => {
        Permutations!();
    };
}

macro_rules! impl_427 {
    () => {
        deps!();
        impl < I > Clone for Permutations < I > where I : Clone + Iterator , I :: Item : Clone , { clone_fields ! (vals , state) ; }
    };
}

impl_427!()