macro_rules! deps {
    () => {
        ArrayLike!();
        ArrayVec!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl < A : ArrayLike > Eq for ArrayVec < A > where A :: Item : Eq { }
    };
}

impl_140!();