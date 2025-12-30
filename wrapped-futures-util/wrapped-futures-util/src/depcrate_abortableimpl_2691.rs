// Generated macro for impl_2691 (impl)
macro_rules! Depcrate_abortableimpl_2691 {
() => {
// Module: crate::abortable
// Provides: {"impl_2691"}
// Dependencies: {}
impl < Fut > Future for Abortable < Fut > where Fut : Future , { type Output = Result < Fut :: Output , Aborted > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { self . try_poll (cx , | fut , cx | fut . poll (cx)) } }
};
}
