macro_rules! deps {
    () => {
        Mutex!();
        OwnedMutexGuard!();
        MutexGuard!();
        OwnedMutexLockFuture!();
        Waiter!();
        MutexLockFuture!();
    };
}

macro_rules! impl_1289 {
    () => {
        deps!();
        impl < T : ? Sized > Mutex < T > { # [doc = " Attempt to acquire the lock immediately."] # [doc = ""] # [doc = " If the lock is currently held, this will return `None`."] pub fn try_lock (& self) -> Option < MutexGuard < '_ , T > > { let old_state = self . state . fetch_or (IS_LOCKED , Ordering :: Acquire) ; if (old_state & IS_LOCKED) == 0 { Some (MutexGuard { mutex : self }) } else { None } } # [doc = " Attempt to acquire the lock immediately."] # [doc = ""] # [doc = " If the lock is currently held, this will return `None`."] pub fn try_lock_owned (self : & Arc < Self >) -> Option < OwnedMutexGuard < T > > { let old_state = self . state . fetch_or (IS_LOCKED , Ordering :: Acquire) ; if (old_state & IS_LOCKED) == 0 { Some (OwnedMutexGuard { mutex : self . clone () }) } else { None } } # [doc = " Acquire the lock asynchronously."] # [doc = ""] # [doc = " This method returns a future that will resolve once the lock has been"] # [doc = " successfully acquired."] pub fn lock (& self) -> MutexLockFuture < '_ , T > { MutexLockFuture { mutex : Some (self) , wait_key : WAIT_KEY_NONE } } # [doc = " Acquire the lock asynchronously."] # [doc = ""] # [doc = " This method returns a future that will resolve once the lock has been"] # [doc = " successfully acquired."] pub fn lock_owned (self : Arc < Self >) -> OwnedMutexLockFuture < T > { OwnedMutexLockFuture { mutex : Some (self) , wait_key : WAIT_KEY_NONE } } # [doc = " Returns a mutable reference to the underlying data."] # [doc = ""] # [doc = " Since this call borrows the `Mutex` mutably, no actual locking needs to"] # [doc = " take place -- the mutable borrow statically guarantees no locks exist."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # futures::executor::block_on(async {"] # [doc = " use futures::lock::Mutex;"] # [doc = ""] # [doc = " let mut mutex = Mutex::new(0);"] # [doc = " *mutex.get_mut() = 10;"] # [doc = " assert_eq!(*mutex.lock().await, 10);"] # [doc = " # });"] # [doc = " ```"] pub fn get_mut (& mut self) -> & mut T { unsafe { & mut * self . value . get () } } fn remove_waker (& self , wait_key : usize , wake_another : bool) { if wait_key != WAIT_KEY_NONE { let mut waiters = self . waiters . lock () . unwrap () ; let removed_waker = waiters . remove (wait_key) ; match removed_waker { Waiter :: Waiting (_) => { } Waiter :: Woken => { if wake_another { if let Some ((_i , waiter)) = waiters . iter_mut () . next () { waiter . wake () ; } } } } if waiters . is_empty () { self . state . fetch_and (! HAS_WAITERS , Ordering :: Relaxed) ; } } } fn unlock (& self) { let old_state = self . state . fetch_and (! IS_LOCKED , Ordering :: AcqRel) ; if (old_state & HAS_WAITERS) != 0 { let mut waiters = self . waiters . lock () . unwrap () ; if let Some ((_i , waiter)) = waiters . iter_mut () . next () { waiter . wake () ; } } } }
    };
}

impl_1289!()