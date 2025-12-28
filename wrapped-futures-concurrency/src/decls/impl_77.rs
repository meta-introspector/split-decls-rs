macro_rules! deps {
    () => {
        WakerArray!();
        ReadinessArray!();
        InlineWakerArray!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl < const N : usize > WakerArray < N > { # [doc = " Create a new instance of `WakerArray`."] pub (crate) fn new () -> Self { let readiness = Arc :: new (Mutex :: new (ReadinessArray :: new ())) ; Self { wakers : array :: from_fn (| i | { Arc :: new (InlineWakerArray :: new (i , readiness . clone ())) . into () }) , readiness , } } pub (crate) fn get (& self , index : usize) -> Option < & Waker > { self . wakers . get (index) } # [doc = " Access the `Readiness`."] pub (crate) fn readiness (& mut self) -> MutexGuard < '_ , ReadinessArray < N > > { self . readiness . as_ref () . lock () . unwrap () } }
    };
}

impl_77!()