// Generated macro for FutureSender (struct)
macro_rules! Depcrate_stream_channelFutureSender {
() => {
// Module: crate::stream::channel
// Provides: {"FutureSender"}
// Dependencies: {}
# [doc = " A future returned by the `Sender::send` method which will resolve to the"] # [doc = " sender once it's available to send another message."] pub struct FutureSender < T , E > where T : Send + 'static , E : Send + 'static , { sender : Option < Sender < T , E > > , data : Option < Result < T , E > > , }
};
}
