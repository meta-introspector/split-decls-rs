macro_rules! deps {
    () => {
        Mutex!();
    };
}

macro_rules! impl_1281 {
    () => {
        deps!();
        impl < T : ? Sized > fmt :: Debug for Mutex < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let state = self . state . load (Ordering :: SeqCst) ; f . debug_struct ("Mutex") . field ("is_locked" , & ((state & IS_LOCKED) != 0)) . field ("has_waiters" , & ((state & HAS_WAITERS) != 0)) . finish () } }
    };
}

impl_1281!();