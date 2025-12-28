macro_rules! deps {
    () => {
        LockLatch!();
        Latch!();
        CountLatchKind!();
        CountLatch!();
        CoreLatch!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl Latch for CountLatch { # [inline] unsafe fn set (this : * const Self) { unsafe { if (* this) . counter . fetch_sub (1 , Ordering :: SeqCst) == 1 { match (* this) . kind { CountLatchKind :: Stealing { ref latch , ref registry , worker_index , } => { let registry = Arc :: clone (registry) ; if CoreLatch :: set (latch) { registry . notify_worker_latch_is_set (worker_index) ; } } CountLatchKind :: Blocking { ref latch } => LockLatch :: set (latch) , } } } } }
    };
}

impl_92!();