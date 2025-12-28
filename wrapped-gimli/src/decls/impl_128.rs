macro_rules! deps {
    () => {
        ArrayLike!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        # [cfg (feature = "read")] impl < T , const N : usize > ArrayLike for Box < [T ; N] > { type Item = T ; fn as_slice (storage : & Self :: Storage) -> & [MaybeUninit < T >] { & storage [..] } fn as_mut_slice (storage : & mut Self :: Storage) -> & mut [MaybeUninit < T >] { & mut storage [..] } }
    };
}

impl_128!()