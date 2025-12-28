macro_rules! deps {
    () => {
        ArrayLike!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl < T , const N : usize > ArrayLike for [T ; N] { type Item = T ; fn as_slice (storage : & Self :: Storage) -> & [MaybeUninit < T >] { storage } fn as_mut_slice (storage : & mut Self :: Storage) -> & mut [MaybeUninit < T >] { storage } }
    };
}

impl_126!();