// Generated macro for AsyncUdpSocket (trait)
macro_rules! Depcrate_runtimeAsyncUdpSocket {
() => {
// Module: crate::runtime
// Provides: {"AsyncUdpSocket"}
// Dependencies: {}
# [doc = " Abstract implementation of a UDP socket for runtime independence"] pub trait AsyncUdpSocket : Send + Sync + Debug + 'static { # [doc = " Create a [`UdpSender`] that can register a single task for write-readiness notifications"] # [doc = " and send a transmit, if ready."] # [doc = ""] # [doc = " A `poll_send` method on a single object can usually store only one [`Waker`] at a time,"] # [doc = " i.e. allow at most one caller to wait for an event. This method allows any number of"] # [doc = " interested tasks to construct their own [`UdpSender`] object. They can all then wait for the"] # [doc = " same event and be notified concurrently, because each [`UdpSender`] can store a separate"] # [doc = " [`Waker`]."] # [doc = ""] # [doc = " [`Waker`]: std::task::Waker"] fn create_sender (& self) -> Pin < Box < dyn UdpSender > > ; # [doc = " Receive UDP datagrams, or register to be woken if receiving may succeed in the future"] fn poll_recv (& mut self , cx : & mut Context , bufs : & mut [IoSliceMut < '_ >] , meta : & mut [RecvMeta] ,) -> Poll < io :: Result < usize > > ; # [doc = " Look up the local IP address and port used by this socket"] fn local_addr (& self) -> io :: Result < SocketAddr > ; # [doc = " Maximum number of datagrams that might be described by a single [`RecvMeta`]"] fn max_receive_segments (& self) -> usize { 1 } # [doc = " Whether datagrams might get fragmented into multiple parts"] # [doc = ""] # [doc = " Sockets should prevent this for best performance. See e.g. the `IPV6_DONTFRAG` socket"] # [doc = " option."] fn may_fragment (& self) -> bool { true } }
};
}
