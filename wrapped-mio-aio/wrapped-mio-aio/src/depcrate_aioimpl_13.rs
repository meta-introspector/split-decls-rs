// Generated macro for impl_13 (impl)
macro_rules! Depcrate_aioimpl_13 {
() => {
// Module: crate::aio
// Provides: {"impl_13"}
// Dependencies: {}
impl < T : Aio > Source < T > { pin_utils :: unsafe_pinned ! (inner : T) ; fn _deregister_raw (& mut self) { let sigev = SigevNotify :: SigevNone ; self . inner . set_sigev_notify (sigev) ; } fn _register_raw (& mut self , kq : RawFd , udata : usize) { let sigev = SigevNotify :: SigevKeventFlags { kq , udata : udata as isize , flags : EventFlag :: EV_ONESHOT , } ; self . inner . set_sigev_notify (sigev) ; } }
};
}
