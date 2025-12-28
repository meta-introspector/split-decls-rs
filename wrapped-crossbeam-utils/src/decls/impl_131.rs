macro_rules! deps {
    () => {
        ShardedLockWriteGuard!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl < T : ? Sized > Drop for ShardedLockWriteGuard < '_ , T > { fn drop (& mut self) { for shard in self . lock . shards . iter () . rev () { unsafe { let dest : * mut _ = shard . write_guard . get () ; let guard = (* dest) . take () ; drop (guard) ; } } } }
    };
}

impl_131!();