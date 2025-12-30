// Generated macro for impl_8 (impl)
macro_rules! Depcrate_eventsimpl_8 {
() => {
// Module: crate::events
// Provides: {"impl_8"}
// Dependencies: {}
impl < 'a > Events < 'a > { pub (crate) fn new (fd : Weak < FdGuard > , buffer : & 'a [u8] , num_bytes : usize) -> Self { Events { fd , buffer , num_bytes , pos : 0 , } } }
};
}
