macro_rules! deps {
    () => {
        AtomicBool!();
        Notify!();
    };
}

macro_rules! impl_307 {
    () => {
        deps!();
        impl Notify { # [doc = " Create a new `Notify`."] pub fn new () -> Notify { Notify { object : rt :: Notify :: new (false , true) , waiting : AtomicBool :: new (false) , } } # [doc = " Notify the waiter."] # [track_caller] pub fn notify (& self) { self . object . notify (location ! ()) ; } # [doc = " Wait for a notification."] # [doc = ""] # [doc = " # Panics"] # [doc = " Panics if multiple threads try to wait on the same `Notify` simultaneously."] # [track_caller] pub fn wait (& self) { self . waiting . compare_exchange (false , true , SeqCst , SeqCst) . expect ("only a single thread may wait on `Notify`") ; self . object . wait (location ! ()) ; self . waiting . store (false , SeqCst) ; } }
    };
}

impl_307!();