macro_rules! deps {
    () => {
        UpdateSeq!();
    };
}

macro_rules! impl_984 {
    () => {
        deps!();
        impl < I , F > ExactSizeIterator for UpdateSeq < I , F > where I : ExactSizeIterator , F : Fn (& mut I :: Item) , { }
    };
}

impl_984!()