macro_rules! MakeMaybeUninit {
    () => {
        pub (crate) struct MakeMaybeUninit < T , const N : usize > (PhantomData < fn () -> T >) ;
    };
}

MakeMaybeUninit!();