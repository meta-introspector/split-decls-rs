macro_rules! deps {
    () => {
        CachePadded!();
        ShardedLock!();
        Shard!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < T > ShardedLock < T > { # [doc = " Creates a new sharded reader-writer lock."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_utils::sync::ShardedLock;"] # [doc = ""] # [doc = " let lock = ShardedLock::new(5);"] # [doc = " ```"] pub fn new (value : T) -> Self { Self { shards : (0 .. NUM_SHARDS) . map (| _ | { CachePadded :: new (Shard { lock : RwLock :: new (()) , write_guard : UnsafeCell :: new (None) , }) }) . collect :: < Box < [_] > > () , value : UnsafeCell :: new (value) , } } # [doc = " Consumes this lock, returning the underlying data."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This method will return an error if the lock is poisoned. A lock gets poisoned when a write"] # [doc = " operation panics."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_utils::sync::ShardedLock;"] # [doc = ""] # [doc = " let lock = ShardedLock::new(String::new());"] # [doc = " {"] # [doc = "     let mut s = lock.write().unwrap();"] # [doc = "     *s = \"modified\".to_owned();"] # [doc = " }"] # [doc = " assert_eq!(lock.into_inner().unwrap(), \"modified\");"] # [doc = " ```"] pub fn into_inner (self) -> LockResult < T > { let is_poisoned = self . is_poisoned () ; let inner = self . value . into_inner () ; if is_poisoned { Err (PoisonError :: new (inner)) } else { Ok (inner) } } }
    };
}

impl_119!();