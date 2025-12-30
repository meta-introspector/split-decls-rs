// Generated macro for impl_50 (impl)
macro_rules! Depcrate_address_channelimpl_50 {
() => {
// Module: crate::address::channel
// Provides: {"impl_50"}
// Dependencies: {}
impl < A , M > Sender < M > for AddressSender < A > where A : Handler < M > , A :: Context : ToEnvelope < A , M > , M :: Result : Send , M : Message + Send + 'static , { fn do_send (& self , msg : M) -> Result < () , SendError < M > > { self . do_send (msg) } fn try_send (& self , msg : M) -> Result < () , SendError < M > > { self . try_send (msg , true) } fn send (& self , msg : M) -> Result < OneshotReceiver < M :: Result > , SendError < M > > { self . send (msg) } fn boxed (& self) -> Box < dyn Sender < M > + Sync > { Box :: new (self . clone ()) } fn hash (& self) -> usize { let hash : * const _ = self . inner . as_ref () ; hash as usize } fn connected (& self) -> bool { self . connected () } fn downgrade (& self) -> Box < dyn WeakSender < M > + Sync + 'static > { Box :: new (WeakAddressSender { inner : Arc :: downgrade (& self . inner) , }) } }
};
}
