macro_rules! deps {
    () => {
        UintSize!();
        Cell!();
        AtomicTargetSize!();
    };
}

macro_rules! impl_409 {
    () => {
        deps!();
        impl < T > Cell < T > { const fn new (seq : usize) -> Self { Self { data : MaybeUninit :: uninit () , sequence : AtomicTargetSize :: new (seq as UintSize) , } } }
    };
}

impl_409!()