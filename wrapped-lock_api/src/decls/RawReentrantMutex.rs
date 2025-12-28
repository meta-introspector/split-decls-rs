macro_rules! deps {
    () => {
        ReentrantMutex!();
        RawMutex!();
    };
}

macro_rules! RawReentrantMutex {
    () => {
        deps!();
        # [doc = " A raw mutex type that wraps another raw mutex to provide reentrancy."] # [doc = ""] # [doc = " Although this has the same methods as the [`RawMutex`] trait, it does"] # [doc = " not implement it, and should not be used in the same way, since this"] # [doc = " mutex can successfully acquire a lock multiple times in the same thread."] # [doc = " Only use this when you know you want a raw mutex that can be locked"] # [doc = " reentrantly; you probably want [`ReentrantMutex`] instead."] pub struct RawReentrantMutex < R , G > { owner : AtomicUsize , lock_count : Cell < usize > , mutex : R , get_thread_id : G , }
    };
}

RawReentrantMutex!()