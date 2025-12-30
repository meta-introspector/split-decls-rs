// Generated macro for impl_134 (impl)
macro_rules! Depcrate_whenimpl_134 {
() => {
// Module: crate::when
// Provides: {"impl_134"}
// Dependencies: {}
impl < T : RuntimeType > IAsyncOperation < T > { # [doc = " Calls `op(result)` when the `IAsyncOperation<T>` completes."] pub fn when < F > (& self , op : F) -> Result < () > where F : FnOnce (Result < T >) + Send + 'static , { Async :: when (self , op) } }
};
}
