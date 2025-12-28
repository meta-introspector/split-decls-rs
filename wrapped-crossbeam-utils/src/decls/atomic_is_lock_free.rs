macro_rules! atomic_is_lock_free {
    () => {
        # [doc = " Returns `true` if operations on `AtomicCell<T>` are lock-free."] const fn atomic_is_lock_free < T > () -> bool { atomic ! { T , _a , true , false } }
    };
}

atomic_is_lock_free!();