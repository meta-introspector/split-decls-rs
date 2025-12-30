// Generated macro for Sender (struct)
macro_rules! Depcrate_stream_channelSender {
() => {
// Module: crate::stream::channel
// Provides: {"Sender"}
// Dependencies: {}
# [doc = " The transmission end of a channel which is used to send values."] # [doc = ""] # [doc = " This is created by the `channel` method in the `stream` module."] pub struct Sender < T , E > where T : Send + 'static , E : Send + 'static , { inner : Arc < Inner < T , E > > , }
};
}
