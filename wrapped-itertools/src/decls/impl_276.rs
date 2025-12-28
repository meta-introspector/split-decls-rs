macro_rules! deps {
    () => {
        IntoChunks!();
    };
}

macro_rules! impl_276 {
    () => {
        deps!();
        impl < I > Clone for IntoChunks < I > where I : Clone + Iterator , I :: Item : Clone , { clone_fields ! (inner , index) ; }
    };
}

impl_276!()