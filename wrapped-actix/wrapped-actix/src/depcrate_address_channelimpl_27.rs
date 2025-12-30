// Generated macro for impl_27 (impl)
macro_rules! Depcrate_address_channelimpl_27 {
() => {
// Module: crate::address::channel
// Provides: {"impl_27"}
// Dependencies: {}
impl < S , M > Sender < M > for Box < S > where S : Sender < M > + ? Sized , M :: Result : Send , M : Message + Send , { fn do_send (& self , msg : M) -> Result < () , SendError < M > > { (* * self) . do_send (msg) } fn try_send (& self , msg : M) -> Result < () , SendError < M > > { (* * self) . try_send (msg) } fn send (& self , msg : M) -> Result < OneshotReceiver < < M as Message > :: Result > , SendError < M > > { (* * self) . send (msg) } fn boxed (& self) -> Box < dyn Sender < M > + Sync > { (* * self) . boxed () } fn hash (& self) -> usize { (* * self) . hash () } fn connected (& self) -> bool { (* * self) . connected () } fn downgrade (& self) -> Box < dyn WeakSender < M > + Sync > { (* * self) . downgrade () } }
};
}
