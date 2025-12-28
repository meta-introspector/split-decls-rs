macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl < T : Clone , const M : usize , const N : usize > From < & [T ; M] > for SmallVec < T , N > { # [inline] fn from (slice : & [T ; M]) -> Self { Self :: from (slice as & [T]) } }
    };
}

impl_128!();