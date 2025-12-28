macro_rules! deps {
    () => {
        FairMutexGuard!();
        MappedMutexGuard!();
        RawFairMutex!();
    };
}

macro_rules! MappedFairMutexGuard {
    () => {
        deps!();
        # [doc = " An RAII mutex guard returned by `FairMutexGuard::map`, which can point to a"] # [doc = " subfield of the protected data."] # [doc = ""] # [doc = " The main difference between `MappedFairMutexGuard` and `FairMutexGuard` is that the"] # [doc = " former doesn't support temporarily unlocking and re-locking, since that"] # [doc = " could introduce soundness issues if the locked object is modified by another"] # [doc = " thread."] pub type MappedFairMutexGuard < 'a , T > = lock_api :: MappedMutexGuard < 'a , RawFairMutex , T > ;
    };
}

MappedFairMutexGuard!();