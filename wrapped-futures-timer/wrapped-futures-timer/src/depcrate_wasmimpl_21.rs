// Generated macro for impl_21 (impl)
macro_rules! Depcrate_wasmimpl_21 {
() => {
// Module: crate::wasm
// Provides: {"impl_21"}
// Dependencies: {}
impl Future for Delay { type Output = () ; fn poll (self : Pin < & mut Self > , cx : & mut Context) -> Poll < Self :: Output > { Pin :: new (& mut * Pin :: into_inner (self) . 0) . poll (cx) } }
};
}
