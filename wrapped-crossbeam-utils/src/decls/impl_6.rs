macro_rules! deps {
    () => {
        SeqLockWriteGuard!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl SeqLockWriteGuard { # [doc = " Releases the lock without incrementing the stamp."] # [inline] pub (crate) fn abort (self) { self . lock . state . store (self . state , Ordering :: Release) ; mem :: forget (self) ; } }
    };
}

impl_6!()