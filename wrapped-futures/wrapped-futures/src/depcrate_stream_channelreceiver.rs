// Generated macro for Receiver (struct)
macro_rules! Depcrate_stream_channelReceiver {
() => {
// Module: crate::stream::channel
// Provides: {"Receiver"}
// Dependencies: {}
# [doc = " The receiving end of a channel which implements the `Stream` trait."] # [doc = ""] # [doc = " This is a concrete implementation of a stream which can be used to represent"] # [doc = " a stream of values being computed elsewhere. This is created by the"] # [doc = " `channel` method in the `stream` module."] pub struct Receiver < T , E > where T : Send + 'static , E : Send + 'static , { inner : Arc < Inner < T , E > > , on_full_token : Option < Token > , }
};
}
