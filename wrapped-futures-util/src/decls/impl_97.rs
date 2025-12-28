macro_rules! deps {
    () => {
        Mutex!();
        FutureOrOutput!();
        Shared!();
        Notifier!();
        Inner!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl < Fut : Future > Shared < Fut > { pub (super) fn new (future : Fut) -> Self { let inner = Inner { future_or_output : UnsafeCell :: new (FutureOrOutput :: Future (future)) , notifier : Arc :: new (Notifier { state : AtomicUsize :: new (IDLE) , wakers : Mutex :: new (Some (Slab :: new ())) , }) , } ; Self { inner : Some (Arc :: new (inner)) , waker_key : NULL_WAKER_KEY } } }
    };
}

impl_97!()