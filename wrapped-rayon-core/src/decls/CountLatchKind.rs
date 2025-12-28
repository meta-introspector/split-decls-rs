macro_rules! deps {
    () => {
        CoreLatch!();
        Registry!();
        LockLatch!();
        ThreadPool!();
    };
}

macro_rules! CountLatchKind {
    () => {
        deps!();
        enum CountLatchKind { # [doc = " A latch for scopes created on a rayon thread which will participate in work"] # [doc = " stealing while it waits for completion. This thread is not necessarily part"] # [doc = " of the same registry as the scope itself!"] Stealing { latch : CoreLatch , # [doc = " If a worker thread in registry A calls `in_place_scope` on a ThreadPool"] # [doc = " with registry B, when a job completes in a thread of registry B, we may"] # [doc = " need to call `notify_worker_latch_is_set()` to wake the thread in registry A."] # [doc = " That means we need a reference to registry A (since at that point we will"] # [doc = " only have a reference to registry B), so we stash it here."] registry : Arc < Registry > , # [doc = " The index of the worker to wake in `registry`"] worker_index : usize , } , # [doc = " A latch for scopes created on a non-rayon thread which will block to wait."] Blocking { latch : LockLatch } , }
    };
}

CountLatchKind!();