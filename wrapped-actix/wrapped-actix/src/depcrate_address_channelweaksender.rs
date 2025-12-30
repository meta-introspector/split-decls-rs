// Generated macro for WeakSender (trait)
macro_rules! Depcrate_address_channelWeakSender {
() => {
// Module: crate::address::channel
// Provides: {"WeakSender"}
// Dependencies: {}
pub trait WeakSender < M > : Send where M :: Result : Send , M : Message + Send , { # [doc = " Attempts to upgrade a `WeakAddressSender<A>` to a [`Sender<M>`]"] # [doc = ""] # [doc = " Returns [`None`] if the actor has since been dropped."] fn upgrade (& self) -> Option < Box < dyn Sender < M > + Sync > > ; fn boxed (& self) -> Box < dyn WeakSender < M > + Sync > ; }
};
}
