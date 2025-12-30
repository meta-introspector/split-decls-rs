// Generated macro for impl_26 (impl)
macro_rules! Depcrate_runtimeimpl_26 {
() => {
// Module: crate::runtime
// Provides: {"impl_26"}
// Dependencies: {}
impl From < tokio :: runtime :: Runtime > for Runtime { fn from (rt : tokio :: runtime :: Runtime) -> Self { Self { local : LocalSet :: new () , rt , } } }
};
}
