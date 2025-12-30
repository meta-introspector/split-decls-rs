// Generated macro for impl_156 (impl)
macro_rules! Depcrate_runtimeimpl_156 {
() => {
// Module: crate::runtime
// Provides: {"impl_156"}
// Dependencies: {}
impl < Socket , MakeWritableFutFn , WriteableFut > UdpSenderHelper < Socket , MakeWritableFutFn , WriteableFut > { # [doc = " Create helper that implements [`UdpSender`] from a socket."] # [doc = ""] # [doc = " Additionally you need to provide what is essentially an async function"] # [doc = " that resolves once the socket is write-ready."] # [doc = ""] # [doc = " See also the bounds on this struct's [`UdpSender`] implementation."] # [cfg (any (feature = "runtime-smol" , feature = "runtime-tokio" ,))] fn new (inner : Socket , make_fut : MakeWritableFutFn) -> Self { Self { socket : inner , make_writable_fut_fn : make_fut , writable_fut : None , } } }
};
}
