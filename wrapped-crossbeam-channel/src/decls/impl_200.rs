macro_rules! deps {
    () => {
        Entry!();
        Operation!();
        Waker!();
        Context!();
        SyncWaker!();
    };
}

macro_rules! impl_200 {
    () => {
        deps!();
        impl SyncWaker { # [doc = " Creates a new `SyncWaker`."] # [inline] pub (crate) fn new () -> Self { Self { inner : Mutex :: new (Waker :: new ()) , is_empty : AtomicBool :: new (true) , } } # [doc = " Registers the current thread with an operation."] # [inline] pub (crate) fn register (& self , oper : Operation , cx : & Context) { let mut inner = self . inner . lock () . unwrap () ; inner . register (oper , cx) ; self . is_empty . store (inner . selectors . is_empty () && inner . observers . is_empty () , Ordering :: SeqCst ,) ; } # [doc = " Unregisters an operation previously registered by the current thread."] # [inline] pub (crate) fn unregister (& self , oper : Operation) -> Option < Entry > { let mut inner = self . inner . lock () . unwrap () ; let entry = inner . unregister (oper) ; self . is_empty . store (inner . selectors . is_empty () && inner . observers . is_empty () , Ordering :: SeqCst ,) ; entry } # [doc = " Attempts to find one thread (not the current one), select its operation, and wake it up."] # [inline] pub (crate) fn notify (& self) { if ! self . is_empty . load (Ordering :: SeqCst) { let mut inner = self . inner . lock () . unwrap () ; if ! self . is_empty . load (Ordering :: SeqCst) { inner . try_select () ; inner . notify () ; self . is_empty . store (inner . selectors . is_empty () && inner . observers . is_empty () , Ordering :: SeqCst ,) ; } } } # [doc = " Registers an operation waiting to be ready."] # [inline] pub (crate) fn watch (& self , oper : Operation , cx : & Context) { let mut inner = self . inner . lock () . unwrap () ; inner . watch (oper , cx) ; self . is_empty . store (inner . selectors . is_empty () && inner . observers . is_empty () , Ordering :: SeqCst ,) ; } # [doc = " Unregisters an operation waiting to be ready."] # [inline] pub (crate) fn unwatch (& self , oper : Operation) { let mut inner = self . inner . lock () . unwrap () ; inner . unwatch (oper) ; self . is_empty . store (inner . selectors . is_empty () && inner . observers . is_empty () , Ordering :: SeqCst ,) ; } # [doc = " Notifies all threads that the channel is disconnected."] # [inline] pub (crate) fn disconnect (& self) { let mut inner = self . inner . lock () . unwrap () ; inner . disconnect () ; self . is_empty . store (inner . selectors . is_empty () && inner . observers . is_empty () , Ordering :: SeqCst ,) ; } }
    };
}

impl_200!();