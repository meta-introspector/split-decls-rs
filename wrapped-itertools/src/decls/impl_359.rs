macro_rules! deps {
    () => {
        MergeBy!();
        PutBack!();
    };
}

macro_rules! impl_359 {
    () => {
        deps!();
        impl < I , J , F > Clone for MergeBy < I , J , F > where I : Iterator , J : Iterator , PutBack < Fuse < I > > : Clone , PutBack < Fuse < J > > : Clone , F : Clone , { clone_fields ! (left , right , cmp_fn) ; }
    };
}

impl_359!();