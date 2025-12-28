macro_rules! deps {
    () => {
        Pending!();
        OwnedMutexGuard!();
        OwnedMutexLockFuture!();
        Ready!();
        Waiter!();
    };
}

macro_rules! impl_1294 {
    () => {
        deps!();
        impl < T : ? Sized > Future for OwnedMutexLockFuture < T > { type Output = OwnedMutexGuard < T > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . get_mut () ; let mutex = this . mutex . as_ref () . expect ("polled OwnedMutexLockFuture after completion") ; if let Some (lock) = mutex . try_lock_owned () { mutex . remove_waker (this . wait_key , false) ; this . mutex = None ; return Poll :: Ready (lock) ; } { let mut waiters = mutex . waiters . lock () . unwrap () ; if this . wait_key == WAIT_KEY_NONE { this . wait_key = waiters . insert (Waiter :: Waiting (cx . waker () . clone ())) ; if waiters . len () == 1 { mutex . state . fetch_or (HAS_WAITERS , Ordering :: Relaxed) ; } } else { waiters [this . wait_key] . register (cx . waker ()) ; } } if let Some (lock) = mutex . try_lock_owned () { mutex . remove_waker (this . wait_key , false) ; this . mutex = None ; return Poll :: Ready (lock) ; } Poll :: Pending } }
    };
}

impl_1294!();