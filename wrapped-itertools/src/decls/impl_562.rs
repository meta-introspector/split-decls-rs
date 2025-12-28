macro_rules! deps {
    () => {
        WithPosition!();
    };
}

macro_rules! impl_562 {
    () => {
        deps!();
        impl < I > Clone for WithPosition < I > where I : Clone + Iterator , I :: Item : Clone , { clone_fields ! (handled_first , peekable) ; }
    };
}

impl_562!();