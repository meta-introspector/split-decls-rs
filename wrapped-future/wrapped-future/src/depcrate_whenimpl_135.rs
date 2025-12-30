// Generated macro for impl_135 (impl)
macro_rules! Depcrate_whenimpl_135 {
() => {
// Module: crate::when
// Provides: {"impl_135"}
// Dependencies: {}
impl < P : RuntimeType > IAsyncActionWithProgress < P > { # [doc = " Calls `op(result)` when the `IAsyncActionWithProgress<P>` completes."] pub fn when < F > (& self , op : F) -> Result < () > where F : FnOnce (Result < () >) + Send + 'static , { Async :: when (self , op) } }
};
}
