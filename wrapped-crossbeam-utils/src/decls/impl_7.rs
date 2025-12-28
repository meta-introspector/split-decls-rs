macro_rules! deps {
    () => {
        SeqLockWriteGuard!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl Drop for SeqLockWriteGuard { # [inline] fn drop (& mut self) { self . lock . state . store (self . state . wrapping_add (2) , Ordering :: Release) ; } }
    };
}

impl_7!()