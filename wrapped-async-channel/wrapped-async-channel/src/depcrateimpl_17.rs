// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
impl < T > Channel < T > { # [doc = " Closes the channel and notifies all blocked operations."] # [doc = ""] # [doc = " Returns `true` if this call has closed the channel and it was not closed already."] fn close (& self) -> bool { if self . queue . close () { self . send_ops . notify (usize :: MAX) ; self . recv_ops . notify (usize :: MAX) ; self . stream_ops . notify (usize :: MAX) ; self . closed_ops . notify (usize :: MAX) ; true } else { false } } }
};
}
