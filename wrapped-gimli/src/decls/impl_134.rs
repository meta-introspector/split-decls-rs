macro_rules! deps {
    () => {
        ArrayVec!();
        ArrayLike!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        impl < A : ArrayLike > Drop for ArrayVec < A > { fn drop (& mut self) { self . clear () ; } }
    };
}

impl_134!()