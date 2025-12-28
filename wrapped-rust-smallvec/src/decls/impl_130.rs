macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl < T , const N : usize , const M : usize > From < [T ; M] > for SmallVec < T , N > { fn from (array : [T ; M]) -> Self { if M > N { Self :: from (Vec :: from (array)) } else { let mut this = Self :: new () ; debug_assert ! (M <= this . capacity ()) ; let array = ManuallyDrop :: new (array) ; unsafe { copy_nonoverlapping (array . as_ptr () , this . as_mut_ptr () , M) ; this . set_len (M) ; } this } } }
    };
}

impl_130!()