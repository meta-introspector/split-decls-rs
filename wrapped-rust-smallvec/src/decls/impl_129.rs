macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl < T : Clone , const M : usize , const N : usize > From < & mut [T ; M] > for SmallVec < T , N > { # [inline] fn from (slice : & mut [T ; M]) -> Self { Self :: from (slice as & [T]) } }
    };
}

impl_129!();