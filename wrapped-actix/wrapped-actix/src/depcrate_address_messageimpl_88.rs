// Generated macro for impl_88 (impl)
macro_rules! Depcrate_address_messageimpl_88 {
() => {
// Module: crate::address::message
// Provides: {"impl_88"}
// Dependencies: {}
impl < S , M > MsgRequest < S , M > where S : Sender < M > , M : Message + Send , M :: Result : Send , { pub (crate) fn new (rx : Option < oneshot :: Receiver < M :: Result > > , info : Option < (S , M) >) -> Self { Self { rx , info , timeout : None , } } # [cfg (test)] pub (crate) fn rx_is_some (& self) -> bool { self . rx . is_some () } # [doc = " Set message delivery timeout"] pub fn timeout (mut self , dur : Duration) -> Self { self . timeout = Some (actix_rt :: time :: sleep (dur)) ; self } }
};
}
