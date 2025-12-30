// Generated macro for UdpSender (trait)
macro_rules! Depcrate_runtimeUdpSender {
() => {
// Module: crate::runtime
// Provides: {"UdpSender"}
// Dependencies: {}
# [doc = " An object for asynchronously writing to an associated [`AsyncUdpSocket`]."] # [doc = ""] # [doc = " Any number of [`UdpSender`]s may exist for a single [`AsyncUdpSocket`]. Each [`UdpSender`] is"] # [doc = " responsible for notifying at most one task for send readiness."] pub trait UdpSender : Send + Sync + Debug + 'static { # [doc = " Send a UDP datagram, or register to be woken if sending may succeed in the future."] # [doc = ""] # [doc = " Usually implementations of this will poll the socket for writability before trying to"] # [doc = " write to them, and retry both if writing fails."] # [doc = ""] # [doc = " Quinn will create multiple [`UdpSender`]s, one for each task it's using it from. Thus it's"] # [doc = " important to poll the underlying socket in a way that doesn't overwrite wakers."] # [doc = ""] # [doc = " A single [`UdpSender`] will be re-used, even if `poll_send` returns `Poll::Ready` once,"] # [doc = " unlike [`Future::poll`], so calling it again after readiness should not panic."] fn poll_send (self : Pin < & mut Self > , transmit : & Transmit , cx : & mut Context ,) -> Poll < io :: Result < () > > ; # [doc = " Maximum number of datagrams that a [`Transmit`] may encode."] fn max_transmit_segments (& self) -> usize { 1 } }
};
}
