macro_rules! deps {
    () => {
        AtomicCell!();
        SeqLock!();
        CachePadded!();
    };
}

macro_rules! lock {
    () => {
        deps!();
        # [doc = " Returns a reference to the global lock associated with the `AtomicCell` at address `addr`."] # [doc = ""] # [doc = " This function is used to protect atomic data which doesn't fit into any of the primitive atomic"] # [doc = " types in `std::sync::atomic`. Operations on such atomics must therefore use a global lock."] # [doc = ""] # [doc = " However, there is not only one global lock but an array of many locks, and one of them is"] # [doc = " picked based on the given address. Having many locks reduces contention and improves"] # [doc = " scalability."] # [inline] # [must_use] fn lock (addr : usize) -> & 'static SeqLock { const LEN : usize = 67 ; const L : CachePadded < SeqLock > = CachePadded :: new (SeqLock :: new ()) ; static LOCKS : [CachePadded < SeqLock > ; LEN] = [L ; LEN] ; & LOCKS [addr % LEN] }
    };
}

lock!();