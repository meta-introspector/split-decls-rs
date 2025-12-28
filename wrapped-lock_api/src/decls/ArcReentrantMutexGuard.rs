macro_rules! deps {
    () => {
        GuardNoSend!();
        Mutex!();
        ReentrantMutexGuard!();
        RawMutex!();
        GetThreadId!();
        ReentrantMutex!();
    };
}

macro_rules! ArcReentrantMutexGuard {
    () => {
        deps!();
        # [doc = " An RAII mutex guard returned by the `Arc` locking operations on `ReentrantMutex`."] # [doc = ""] # [doc = " This is similar to the `ReentrantMutexGuard` struct, except instead of using a reference to unlock the"] # [doc = " `Mutex` it uses an `Arc<ReentrantMutex>`. This has several advantages, most notably that it has an `'static`"] # [doc = " lifetime."] # [cfg (feature = "arc_lock")] # [clippy :: has_significant_drop] # [must_use = "if unused the ReentrantMutex will immediately unlock"] pub struct ArcReentrantMutexGuard < R : RawMutex , G : GetThreadId , T : ? Sized > { remutex : Arc < ReentrantMutex < R , G , T > > , marker : PhantomData < GuardNoSend > , }
    };
}

ArcReentrantMutexGuard!();