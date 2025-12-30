// Generated macro for spawn (function)
macro_rules! Depcrate_executorspawn {
() => {
// Module: crate::executor
// Provides: {"spawn"}
// Dependencies: {}
# [doc = " Spawns a future on the executor."] # [cfg_attr (not (any (feature = "alloc-stats" , feature = "shell" , feature = "net" , feature = "vsock")) , expect (dead_code))] pub (crate) fn spawn < F > (future : F) where F : Future < Output = () > + Send + 'static , { core_local :: ex () . spawn (AsyncTask :: new (future)) . detach () ; }
};
}
