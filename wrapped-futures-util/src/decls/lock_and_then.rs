macro_rules! deps {
    () => {
        BiLock!();
    };
}

macro_rules! lock_and_then {
    () => {
        deps!();
        fn lock_and_then < T , U , E , F > (lock : & BiLock < T > , cx : & mut Context < '_ > , f : F) -> Poll < Result < U , E > > where F : FnOnce (Pin < & mut T > , & mut Context < '_ >) -> Poll < Result < U , E > > , { let mut l = ready ! (lock . poll_lock (cx)) ; f (l . as_pin_mut () , cx) }
    };
}

lock_and_then!();