macro_rules! Shard {
    () => {
        # [doc = " A shard containing a single reader-writer lock."] struct Shard { # [doc = " The inner reader-writer lock."] lock : RwLock < () > , # [doc = " The write-guard keeping this shard locked."] # [doc = ""] # [doc = " Write operations will lock each shard and store the guard here. These guards get dropped at"] # [doc = " the same time the big guard is dropped."] write_guard : UnsafeCell < Option < RwLockWriteGuard < 'static , () > > > , }
    };
}

Shard!();