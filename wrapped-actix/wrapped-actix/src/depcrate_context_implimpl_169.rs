// Generated macro for impl_169 (impl)
macro_rules! Depcrate_context_implimpl_169 {
() => {
// Module: crate::context_impl
// Provides: {"impl_169"}
// Dependencies: {}
impl < A , C > Drop for ContextFut < A , C > where C : AsyncContextParts < A > + Unpin , A : Actor < Context = C > , { fn drop (& mut self) { if self . alive () { self . ctx . parts () . stop () ; let waker = futures_task :: noop_waker () ; let mut cx = std :: task :: Context :: from_waker (& waker) ; let _ = Pin :: new (self) . poll (& mut cx) ; } } }
};
}
