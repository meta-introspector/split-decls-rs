// Generated macro for impl_157 (impl)
macro_rules! Depcrate_runtimeimpl_157 {
() => {
// Module: crate::runtime
// Provides: {"impl_157"}
// Dependencies: {}
impl < Socket , MakeWritableFutFn , WritableFut > super :: UdpSender for UdpSenderHelper < Socket , MakeWritableFutFn , WritableFut > where Socket : UdpSenderHelperSocket , MakeWritableFutFn : Fn (& Socket) -> WritableFut + Send + Sync + 'static , WritableFut : Future < Output = io :: Result < () > > + Send + Sync + 'static , { fn poll_send (self : Pin < & mut Self > , transmit : & udp :: Transmit , cx : & mut Context ,) -> Poll < io :: Result < () > > { let mut this = self . project () ; loop { if this . writable_fut . is_none () { this . writable_fut . set (Some ((this . make_writable_fut_fn) (this . socket))) ; } let result = std :: task :: ready ! (this . writable_fut . as_mut () . as_pin_mut () . unwrap () . poll (cx)) ; this . writable_fut . set (None) ; result ? ; match this . socket . try_send (transmit) { Err (e) if e . kind () == io :: ErrorKind :: WouldBlock => continue , result => return Poll :: Ready (result) , } } } fn max_transmit_segments (& self) -> usize { self . socket . max_transmit_segments () } }
};
}
