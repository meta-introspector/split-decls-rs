// Generated macro for Connecting (struct)
macro_rules! Depcrate_client_legacy_poolConnecting {
() => {
// Module: crate::client::legacy::pool
// Provides: {"Connecting"}
// Dependencies: {}
# [allow (missing_debug_implementations)] pub struct Connecting < T : Poolable , K : Key > { key : K , pool : WeakOpt < Mutex < PoolInner < T , K > > > , }
};
}
