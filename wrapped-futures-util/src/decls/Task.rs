macro_rules! deps {
    () => {
        ReadyToRunQueue!();
    };
}

macro_rules! Task {
    () => {
        deps!();
        pub (super) struct Task < Fut > { pub (super) future : UnsafeCell < Option < Fut > > , pub (super) next_all : AtomicPtr < Task < Fut > > , pub (super) prev_all : UnsafeCell < * const Task < Fut > > , pub (super) len_all : UnsafeCell < usize > , pub (super) next_ready_to_run : AtomicPtr < Task < Fut > > , pub (super) ready_to_run_queue : Weak < ReadyToRunQueue < Fut > > , pub (super) queued : AtomicBool , pub (super) woken : AtomicBool , }
    };
}

Task!()