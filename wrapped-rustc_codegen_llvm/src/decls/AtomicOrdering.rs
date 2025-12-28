macro_rules! AtomicOrdering {
    () => {
        # [doc = " LLVMAtomicOrdering"] # [derive (Copy , Clone)] # [repr (C)] pub (crate) enum AtomicOrdering { # [allow (dead_code)] NotAtomic = 0 , # [allow (dead_code)] Unordered = 1 , Monotonic = 2 , Acquire = 4 , Release = 5 , AcquireRelease = 6 , SequentiallyConsistent = 7 , }
    };
}

AtomicOrdering!();