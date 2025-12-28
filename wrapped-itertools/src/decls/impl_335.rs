macro_rules! deps {
    () => {
        KMergeBy!();
    };
}

macro_rules! impl_335 {
    () => {
        deps!();
        impl < I , F > Clone for KMergeBy < I , F > where I : Iterator + Clone , I :: Item : Clone , F : Clone , { clone_fields ! (heap , less_than) ; }
    };
}

impl_335!();