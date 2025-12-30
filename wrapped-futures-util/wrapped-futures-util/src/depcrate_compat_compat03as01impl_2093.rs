// Generated macro for impl_2093 (impl)
macro_rules! Depcrate_compat_compat03as01impl_2093 {
() => {
// Module: crate::compat::compat03as01
// Provides: {"impl_2093"}
// Dependencies: {}
impl < Fut > Future01 for Compat < Fut > where Fut : TryFuture03 + Unpin , { type Item = Fut :: Ok ; type Error = Fut :: Error ; fn poll (& mut self) -> Poll01 < Self :: Item , Self :: Error > { with_context (self , | inner , cx | poll_03_to_01 (inner . try_poll (cx))) } }
};
}
