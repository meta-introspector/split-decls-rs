// Generated macro for impl_2683 (impl)
macro_rules! Depcrate_abortableimpl_2683 {
() => {
// Module: crate::abortable
// Provides: {"impl_2683"}
// Dependencies: {}
impl AbortRegistration { # [doc = " Create an [`AbortHandle`] from the given [`AbortRegistration`]."] # [doc = ""] # [doc = " The created [`AbortHandle`] is functionally the same as any other"] # [doc = " [`AbortHandle`]s that are associated with the same [`AbortRegistration`],"] # [doc = " such as the one created by [`AbortHandle::new_pair`]."] pub fn handle (& self) -> AbortHandle { AbortHandle { inner : self . inner . clone () } } }
};
}
