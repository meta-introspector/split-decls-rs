macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < T : Clone , const N : usize > From < & mut [T] > for SmallVec < T , N > { # [inline] fn from (slice : & mut [T]) -> Self { Self :: from (slice as & [T]) } }
    };
}

impl_57!()