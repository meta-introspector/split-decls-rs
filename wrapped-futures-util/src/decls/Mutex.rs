macro_rules! deps {
    () => {
        Waiter!();
    };
}

macro_rules! Mutex {
    () => {
        deps!();
        # [doc = " A futures-aware mutex."] # [doc = ""] # [doc = " # Fairness"] # [doc = ""] # [doc = " This mutex provides no fairness guarantees. Tasks may not acquire the mutex"] # [doc = " in the order that they requested the lock, and it's possible for a single task"] # [doc = " which repeatedly takes the lock to starve other tasks, which may be left waiting"] # [doc = " indefinitely."] pub struct Mutex < T : ? Sized > { state : AtomicUsize , waiters : StdMutex < Slab < Waiter > > , value : UnsafeCell < T > , }
    };
}

Mutex!();