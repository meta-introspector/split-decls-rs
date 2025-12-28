macro_rules! deps {
    () => {
        Tee!();
    };
}

macro_rules! impl_500 {
    () => {
        deps!();
        impl < I > ExactSizeIterator for Tee < I > where I : ExactSizeIterator , I :: Item : Clone , { }
    };
}

impl_500!()