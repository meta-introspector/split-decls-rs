// Generated macro for State (struct)
macro_rules! DepcrateState {
() => {
// Module: crate
// Provides: {"State"}
// Dependencies: {}
# [doc = " The state of a executor."] struct State { # [doc = " The global queue."] queue : ConcurrentQueue < Runnable > , # [doc = " Local queues created by runners."] local_queues : RwLock < Vec < Arc < ConcurrentQueue < Runnable > > > > , # [doc = " Set to `true` when a sleeping ticker is notified or no tickers are sleeping."] notified : AtomicBool , # [doc = " A list of sleeping tickers."] sleepers : Mutex < Sleepers > , # [doc = " Currently active tasks."] active : Mutex < Slab < Waker > > , }
};
}
