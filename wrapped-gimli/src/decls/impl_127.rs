macro_rules! impl_127 {
    () => {
        # [cfg (feature = "read")] unsafe impl < T , const N : usize > Sealed for Box < [T ; N] > { type Storage = Box < [MaybeUninit < T > ; N] > ; fn new_storage () -> Self :: Storage { Box :: new (unsafe { MaybeUninit :: uninit () . assume_init () }) } }
    };
}

impl_127!()