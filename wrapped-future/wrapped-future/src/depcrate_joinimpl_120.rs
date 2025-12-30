// Generated macro for impl_120 (impl)
macro_rules! Depcrate_joinimpl_120 {
() => {
// Module: crate::join
// Provides: {"impl_120"}
// Dependencies: {}
impl < T : RuntimeType > IAsyncOperation < T > { # [doc = " Waits for the `IAsyncOperation<T>` to finish."] pub fn join (& self) -> Result < T > { Async :: join (self) } }
};
}
