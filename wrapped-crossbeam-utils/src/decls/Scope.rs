macro_rules! deps {
    () => {
        SharedVec!();
        WaitGroup!();
        SharedOption!();
    };
}

macro_rules! Scope {
    () => {
        deps!();
        # [doc = " A scope for spawning threads."] pub struct Scope < 'env > { # [doc = " The list of the thread join handles."] handles : SharedVec < SharedOption < thread :: JoinHandle < () > > > , # [doc = " Used to wait until all subscopes all dropped."] wait_group : WaitGroup , # [doc = " Borrows data with invariant lifetime `'env`."] _marker : PhantomData < & 'env mut & 'env () > , }
    };
}

Scope!();