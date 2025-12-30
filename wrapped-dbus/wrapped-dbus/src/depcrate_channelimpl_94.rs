// Generated macro for impl_94 (impl)
macro_rules! Depcrate_channelimpl_94 {
() => {
// Module: crate::channel
// Provides: {"impl_94"}
// Dependencies: {}
# [doc = " Use in case you don't want the send the message, but just collect it instead."] impl Sender for std :: sync :: Mutex < Vec < Message > > { fn send (& self , msg : Message) -> Result < u32 , () > { self . lock () . unwrap () . push (msg) ; Ok (0) } }
};
}
