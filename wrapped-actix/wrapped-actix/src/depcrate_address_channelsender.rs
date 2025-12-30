// Generated macro for Sender (trait)
macro_rules! Depcrate_address_channelSender {
() => {
// Module: crate::address::channel
// Provides: {"Sender"}
// Dependencies: {}
pub trait Sender < M > : Send where M :: Result : Send , M : Message + Send , { fn do_send (& self , msg : M) -> Result < () , SendError < M > > ; fn try_send (& self , msg : M) -> Result < () , SendError < M > > ; fn send (& self , msg : M) -> Result < OneshotReceiver < M :: Result > , SendError < M > > ; fn boxed (& self) -> Box < dyn Sender < M > + Sync > ; fn hash (& self) -> usize ; fn connected (& self) -> bool ; # [doc = " Returns a downgraded sender, where the sender is downgraded into its weak counterpart."] fn downgrade (& self) -> Box < dyn WeakSender < M > + Sync + 'static > ; }
};
}
