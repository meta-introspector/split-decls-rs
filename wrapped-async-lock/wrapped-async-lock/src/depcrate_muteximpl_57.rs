// Generated macro for impl_57 (impl)
macro_rules! Depcrate_muteximpl_57 {
() => {
// Module: crate::mutex
// Provides: {"impl_57"}
// Dependencies: {}
impl < T : ? Sized , B : Borrow < Mutex < T > > > AcquireSlow < B , T > { # [doc = " Create a new `AcquireSlow` future."] # [cold] fn new (mutex : B) -> Self { AcquireSlow { mutex : Some (mutex) , listener : None , start : Start { # [cfg (all (feature = "std" , not (target_family = "wasm")))] start : None , } , starved : false , _marker : PhantomData , _pin : PhantomPinned , } } # [doc = " Take the mutex reference out, decrementing the counter if necessary."] fn take_mutex (self : Pin < & mut Self >) -> Option < B > { let this = self . project () ; let mutex = this . mutex . take () ; if * this . starved { if let Some (mutex) = mutex . as_ref () { mutex . borrow () . state . fetch_sub (2 , Ordering :: Release) ; } } mutex } }
};
}
