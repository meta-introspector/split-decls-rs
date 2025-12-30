// Generated macro for impl_122 (impl)
macro_rules! Depcrate_joinimpl_122 {
() => {
// Module: crate::join
// Provides: {"impl_122"}
// Dependencies: {}
impl < T : RuntimeType , P : RuntimeType > IAsyncOperationWithProgress < T , P > { # [doc = " Waits for the `IAsyncOperationWithProgress<T, P>` to finish."] pub fn join (& self) -> Result < T > { Async :: join (self) } }
};
}
