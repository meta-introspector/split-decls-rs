// Generated macro for impl_136 (impl)
macro_rules! Depcrate_whenimpl_136 {
() => {
// Module: crate::when
// Provides: {"impl_136"}
// Dependencies: {}
impl < T : RuntimeType , P : RuntimeType > IAsyncOperationWithProgress < T , P > { # [doc = " Calls `op(result)` when the `IAsyncOperationWithProgress<T, P>` completes."] pub fn when < F > (& self , op : F) -> Result < () > where F : FnOnce (Result < T >) + Send + 'static , { Async :: when (self , op) } }
};
}
