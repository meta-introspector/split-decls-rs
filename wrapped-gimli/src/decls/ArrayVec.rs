macro_rules! deps {
    () => {
        ArrayLike!();
    };
}

macro_rules! ArrayVec {
    () => {
        deps!();
        pub (crate) struct ArrayVec < A : ArrayLike > { storage : A :: Storage , len : usize , }
    };
}

ArrayVec!()