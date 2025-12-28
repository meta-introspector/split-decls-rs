macro_rules! deps {
    () => {
        Shard!();
        CachePadded!();
    };
}

macro_rules! ShardedLock {
    () => {
        deps!();
        # [doc = " A sharded reader-writer lock."] # [doc = ""] # [doc = " This lock is equivalent to [`RwLock`], except read operations are faster and write operations"] # [doc = " are slower."] # [doc = ""] # [doc = " A `ShardedLock` is internally made of a list of *shards*, each being a [`RwLock`] occupying a"] # [doc = " single cache line. Read operations will pick one of the shards depending on the current thread"] # [doc = " and lock it. Write operations need to lock all shards in succession."] # [doc = ""] # [doc = " By splitting the lock into shards, concurrent read operations will in most cases choose"] # [doc = " different shards and thus update different cache lines, which is good for scalability. However,"] # [doc = " write operations need to do more work and are therefore slower than usual."] # [doc = ""] # [doc = " The priority policy of the lock is dependent on the underlying operating system's"] # [doc = " implementation, and this type does not guarantee that any particular policy will be used."] # [doc = ""] # [doc = " # Poisoning"] # [doc = ""] # [doc = " A `ShardedLock`, like [`RwLock`], will become poisoned on a panic. Note that it may only be"] # [doc = " poisoned if a panic occurs while a write operation is in progress. If a panic occurs in any"] # [doc = " read operation, the lock will not be poisoned."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_utils::sync::ShardedLock;"] # [doc = ""] # [doc = " let lock = ShardedLock::new(5);"] # [doc = ""] # [doc = " // Any number of read locks can be held at once."] # [doc = " {"] # [doc = "     let r1 = lock.read().unwrap();"] # [doc = "     let r2 = lock.read().unwrap();"] # [doc = "     assert_eq!(*r1, 5);"] # [doc = "     assert_eq!(*r2, 5);"] # [doc = " } // Read locks are dropped at this point."] # [doc = ""] # [doc = " // However, only one write lock may be held."] # [doc = " {"] # [doc = "     let mut w = lock.write().unwrap();"] # [doc = "     *w += 1;"] # [doc = "     assert_eq!(*w, 6);"] # [doc = " } // Write lock is dropped here."] # [doc = " ```"] # [doc = ""] # [doc = " [`RwLock`]: std::sync::RwLock"] pub struct ShardedLock < T : ? Sized > { # [doc = " A list of locks protecting the internal data."] shards : Box < [CachePadded < Shard >] > , # [doc = " The internal data."] value : UnsafeCell < T > , }
    };
}

ShardedLock!();