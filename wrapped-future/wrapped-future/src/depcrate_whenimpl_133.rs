// Generated macro for impl_133 (impl)
macro_rules! Depcrate_whenimpl_133 {
() => {
// Module: crate::when
// Provides: {"impl_133"}
// Dependencies: {}
impl IAsyncAction { # [doc = " Calls `op(result)` when the `IAsyncAction` completes."] pub fn when < F > (& self , op : F) -> Result < () > where F : FnOnce (Result < () >) + Send + 'static , { Async :: when (self , op) } }
};
}
