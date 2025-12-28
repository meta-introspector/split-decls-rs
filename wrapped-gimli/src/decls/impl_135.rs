macro_rules! deps {
    () => {
        ArrayLike!();
        ArrayVec!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        impl < A : ArrayLike > Default for ArrayVec < A > { fn default () -> Self { Self :: new () } }
    };
}

impl_135!();