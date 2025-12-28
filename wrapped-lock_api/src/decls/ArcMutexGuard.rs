macro_rules! deps {
    () => {
        Mutex!();
        MutexGuard!();
        RawMutex!();
    };
}

macro_rules! ArcMutexGuard {
    () => {
        deps!();
        # [doc = " An RAII mutex guard returned by the `Arc` locking operations on `Mutex`."] # [doc = ""] # [doc = " This is similar to the `MutexGuard` struct, except instead of using a reference to unlock the `Mutex` it"] # [doc = " uses an `Arc<Mutex>`. This has several advantages, most notably that it has an `'static` lifetime."] # [cfg (feature = "arc_lock")] # [clippy :: has_significant_drop] # [must_use = "if unused the Mutex will immediately unlock"] pub struct ArcMutexGuard < R : RawMutex , T : ? Sized > { mutex : Arc < Mutex < R , T > > , marker : PhantomData < * const () > , }
    };
}

ArcMutexGuard!();