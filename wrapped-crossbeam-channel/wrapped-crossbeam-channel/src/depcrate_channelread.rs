// Generated macro for read (function)
macro_rules! Depcrate_channelread {
() => {
// Module: crate::channel
// Provides: {"read"}
// Dependencies: {}
# [doc = " Reads a message from the channel."] pub (crate) unsafe fn read < T > (r : & Receiver < T > , token : & mut Token) -> Result < T , () > { unsafe { match & r . flavor { ReceiverFlavor :: Array (chan) => chan . read (token) , ReceiverFlavor :: List (chan) => chan . read (token) , ReceiverFlavor :: Zero (chan) => chan . read (token) , ReceiverFlavor :: At (chan) => { mem :: transmute_copy :: < Result < Instant , () > , Result < T , () > > (& chan . read (token)) } ReceiverFlavor :: Tick (chan) => { mem :: transmute_copy :: < Result < Instant , () > , Result < T , () > > (& chan . read (token)) } ReceiverFlavor :: Never (chan) => chan . read (token) , } } }
};
}
