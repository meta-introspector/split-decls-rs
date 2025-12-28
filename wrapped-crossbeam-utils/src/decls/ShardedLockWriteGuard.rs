macro_rules! deps {
    () => {
        ShardedLock!();
    };
}

macro_rules! ShardedLockWriteGuard {
    () => {
        deps!();
        # [doc = " A guard used to release the exclusive write access of a [`ShardedLock`] when dropped."] # [clippy :: has_significant_drop] pub struct ShardedLockWriteGuard < 'a , T : ? Sized > { lock : & 'a ShardedLock < T > , _marker : PhantomData < RwLockWriteGuard < 'a , T > > , }
    };
}

ShardedLockWriteGuard!()