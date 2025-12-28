macro_rules! deps {
    () => {
        Update!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl < I , F > ExactSizeIterator for Update < I , F > where I : ExactSizeIterator , F : FnMut (& mut I :: Item) , { }
    };
}

impl_143!();