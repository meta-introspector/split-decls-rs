// Generated macro for impl_121 (impl)
macro_rules! Depcrate_joinimpl_121 {
() => {
// Module: crate::join
// Provides: {"impl_121"}
// Dependencies: {}
impl < P : RuntimeType > IAsyncActionWithProgress < P > { # [doc = " Waits for the `IAsyncActionWithProgress<P>` to finish."] pub fn join (& self) -> Result < () > { Async :: join (self) } }
};
}
