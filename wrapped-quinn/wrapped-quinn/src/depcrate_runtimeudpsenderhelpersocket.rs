// Generated macro for UdpSenderHelperSocket (trait)
macro_rules! Depcrate_runtimeUdpSenderHelperSocket {
() => {
// Module: crate::runtime
// Provides: {"UdpSenderHelperSocket"}
// Dependencies: {}
# [doc = " Parts of the [`UdpSender`] trait that aren't asynchronous or require storing wakers."] # [doc = ""] # [doc = " This trait is used by [`UdpSenderHelper`] to help construct [`UdpSender`]s."] trait UdpSenderHelperSocket : Send + Sync + 'static { # [doc = " Try to send a transmit, if the socket happens to be write-ready."] # [doc = ""] # [doc = " If not write-ready, this is allowed to return [`std::io::ErrorKind::WouldBlock`]."] # [doc = ""] # [doc = " The [`UdpSenderHelper`] will use this to implement [`UdpSender::poll_send`]."] fn try_send (& self , transmit : & udp :: Transmit) -> io :: Result < () > ; # [doc = " See [`UdpSender::max_transmit_segments`]."] fn max_transmit_segments (& self) -> usize ; }
};
}
