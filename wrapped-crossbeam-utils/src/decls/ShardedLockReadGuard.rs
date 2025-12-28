macro_rules! deps {
    () => {
        ShardedLock!();
    };
}

macro_rules! ShardedLockReadGuard {
    () => {
        deps!();
        # [doc = " A guard used to release the shared read access of a [`ShardedLock`] when dropped."] # [clippy :: has_significant_drop] pub struct ShardedLockReadGuard < 'a , T : ? Sized > { lock : & 'a ShardedLock < T > , _guard : RwLockReadGuard < 'a , () > , _marker : PhantomData < RwLockReadGuard < 'a , T > > , }
    };
}

ShardedLockReadGuard!()