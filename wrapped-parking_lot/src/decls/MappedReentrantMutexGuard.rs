macro_rules! deps {
    () => {
        RawThreadId!();
        ReentrantMutexGuard!();
        RawMutex!();
    };
}

macro_rules! MappedReentrantMutexGuard {
    () => {
        deps!();
        # [doc = " An RAII mutex guard returned by `ReentrantMutexGuard::map`, which can point to a"] # [doc = " subfield of the protected data."] # [doc = ""] # [doc = " The main difference between `MappedReentrantMutexGuard` and `ReentrantMutexGuard` is that the"] # [doc = " former doesn't support temporarily unlocking and re-locking, since that"] # [doc = " could introduce soundness issues if the locked object is modified by another"] # [doc = " thread."] pub type MappedReentrantMutexGuard < 'a , T > = lock_api :: MappedReentrantMutexGuard < 'a , RawMutex , RawThreadId , T > ;
    };
}

MappedReentrantMutexGuard!();