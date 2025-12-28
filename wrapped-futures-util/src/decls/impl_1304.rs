macro_rules! deps {
    () => {
        MutexLockFuture!();
        Waiter!();
        MutexGuard!();
        Ready!();
        Pending!();
    };
}

macro_rules! impl_1304 {
    () => {
        deps!();
        impl < 'a , T : ? Sized > Future for MutexLockFuture < 'a , T > { type Output = MutexGuard < 'a , T > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mutex = self . mutex . expect ("polled MutexLockFuture after completion") ; if let Some (lock) = mutex . try_lock () { mutex . remove_waker (self . wait_key , false) ; self . mutex = None ; return Poll :: Ready (lock) ; } { let mut waiters = mutex . waiters . lock () . unwrap () ; if self . wait_key == WAIT_KEY_NONE { self . wait_key = waiters . insert (Waiter :: Waiting (cx . waker () . clone ())) ; if waiters . len () == 1 { mutex . state . fetch_or (HAS_WAITERS , Ordering :: Relaxed) ; } } else { waiters [self . wait_key] . register (cx . waker ()) ; } } if let Some (lock) = mutex . try_lock () { mutex . remove_waker (self . wait_key , false) ; self . mutex = None ; return Poll :: Ready (lock) ; } Poll :: Pending } }
    };
}

impl_1304!()