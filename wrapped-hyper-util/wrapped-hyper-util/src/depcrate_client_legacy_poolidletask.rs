// Generated macro for IdleTask (struct)
macro_rules! Depcrate_client_legacy_poolIdleTask {
() => {
// Module: crate::client::legacy::pool
// Provides: {"IdleTask"}
// Dependencies: {}
struct IdleTask < T , K : Key > { timer : Timer , duration : Duration , pool : WeakOpt < Mutex < PoolInner < T , K > > > , pool_drop_notifier : oneshot :: Receiver < Infallible > , }
};
}
