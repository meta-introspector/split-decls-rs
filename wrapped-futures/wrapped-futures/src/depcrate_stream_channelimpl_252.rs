// Generated macro for impl_252 (impl)
macro_rules! Depcrate_stream_channelimpl_252 {
() => {
// Module: crate::stream::channel
// Provides: {"impl_252"}
// Dependencies: {}
impl < T , E > Sender < T , E > where T : Send + 'static , E : Send + 'static , { # [doc = " Sends a new value along this channel to the receiver."] # [doc = ""] # [doc = " This method consumes the sender and returns a future which will resolve"] # [doc = " to the sender again when the value sent has been consumed."] pub fn send (self , t : Result < T , E >) -> FutureSender < T , E > { FutureSender { sender : Some (self) , data : Some (t) , } } }
};
}
