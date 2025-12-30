// Generated macro for Pooled (struct)
macro_rules! Depcrate_client_legacy_poolPooled {
() => {
// Module: crate::client::legacy::pool
// Provides: {"Pooled"}
// Dependencies: {}
# [doc = " A wrapped poolable value that tries to reinsert to the Pool on Drop."] pub struct Pooled < T : Poolable , K : Key > { value : Option < T > , is_reused : bool , key : K , pool : WeakOpt < Mutex < PoolInner < T , K > > > , }
};
}
