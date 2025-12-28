macro_rules! impl_125 {
    () => {
        unsafe impl < T , const N : usize > Sealed for [T ; N] { type Storage = [MaybeUninit < T > ; N] ; fn new_storage () -> Self :: Storage { unsafe { MaybeUninit :: uninit () . assume_init () } } }
    };
}

impl_125!()