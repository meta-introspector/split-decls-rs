macro_rules! deps {
    () => {
        Backoff!();
        SeqLockWriteGuard!();
        SeqLock!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl SeqLock { pub (crate) const fn new () -> Self { Self { state : AtomicUsize :: new (0) , } } # [doc = " If not locked, returns the current stamp."] # [doc = ""] # [doc = " This method should be called before optimistic reads."] # [inline] pub (crate) fn optimistic_read (& self) -> Option < usize > { let state = self . state . load (Ordering :: Acquire) ; if state == 1 { None } else { Some (state) } } # [doc = " Returns `true` if the current stamp is equal to `stamp`."] # [doc = ""] # [doc = " This method should be called after optimistic reads to check whether they are valid. The"] # [doc = " argument `stamp` should correspond to the one returned by method `optimistic_read`."] # [inline] pub (crate) fn validate_read (& self , stamp : usize) -> bool { atomic :: fence (Ordering :: Acquire) ; self . state . load (Ordering :: Relaxed) == stamp } # [doc = " Grabs the lock for writing."] # [inline] pub (crate) fn write (& 'static self) -> SeqLockWriteGuard { let backoff = Backoff :: new () ; loop { let previous = self . state . swap (1 , Ordering :: Acquire) ; if previous != 1 { atomic :: fence (Ordering :: Release) ; return SeqLockWriteGuard { lock : self , state : previous , } ; } backoff . snooze () ; } } }
    };
}

impl_4!()