macro_rules! deps {
    () => {
        Task!();
    };
}

macro_rules! ReadyToRunQueue {
    () => {
        deps!();
        pub (super) struct ReadyToRunQueue < Fut > { pub (super) waker : AtomicWaker , pub (super) head : AtomicPtr < Task < Fut > > , pub (super) tail : UnsafeCell < * const Task < Fut > > , pub (super) stub : Arc < Task < Fut > > , }
    };
}

ReadyToRunQueue!();