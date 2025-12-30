// Generated macro for impl_93 (impl)
macro_rules! Depcrate_channelimpl_93 {
() => {
// Module: crate::channel
// Provides: {"impl_93"}
// Dependencies: {}
# [doc = " Use in case you don't want the send the message, but just collect it instead."] impl Sender for std :: cell :: RefCell < Vec < Message > > { fn send (& self , msg : Message) -> Result < u32 , () > { self . borrow_mut () . push (msg) ; Ok (0) } }
};
}
