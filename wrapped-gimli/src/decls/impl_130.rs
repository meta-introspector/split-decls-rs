macro_rules! deps {
    () => {
        ArrayLike!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        # [cfg (feature = "read")] impl < T > ArrayLike for Vec < T > { type Item = T ; fn as_slice (storage : & Self :: Storage) -> & [MaybeUninit < T >] { storage } fn as_mut_slice (storage : & mut Self :: Storage) -> & mut [MaybeUninit < T >] { storage } }
    };
}

impl_130!();