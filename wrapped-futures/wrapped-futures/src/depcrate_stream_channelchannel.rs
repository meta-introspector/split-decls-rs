// Generated macro for channel (function)
macro_rules! Depcrate_stream_channelchannel {
() => {
// Module: crate::stream::channel
// Provides: {"channel"}
// Dependencies: {}
# [doc = " Creates an in-memory channel implementation of the `Stream` trait."] # [doc = ""] # [doc = " This method creates a concrete implementation of the `Stream` trait which"] # [doc = " can be used to send values across threads in a streaming fashion. This"] # [doc = " channel is unique in that it implements back pressure to ensure that the"] # [doc = " sender never outpaces the receiver. The `Sender::send` method will only"] # [doc = " allow sending one message and the next message can only be sent once the"] # [doc = " first was consumed."] # [doc = ""] # [doc = " The `Receiver` returned implements the `Stream` trait and has access to any"] # [doc = " number of the associated combinators for transforming the result."] pub fn channel < T , E > () -> (Sender < T , E > , Receiver < T , E >) where T : Send + 'static , E : Send + 'static , { let inner = Arc :: new (Inner { slot : Slot :: new (None) , receiver_gone : AtomicBool :: new (false) , }) ; let sender = Sender { inner : inner . clone () , } ; let receiver = Receiver { inner : inner , on_full_token : None , } ; (sender , receiver) }
};
}
