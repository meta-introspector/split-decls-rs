// Generated macro for Send (struct)
macro_rules! Depcrate_sink_sendSend {
() => {
// Module: crate::sink::send
// Provides: {"Send"}
// Dependencies: {}
# [doc = " Future for the [`send`](super::SinkExt::send) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Send < 'a , Si : ? Sized , Item > { feed : Feed < 'a , Si , Item > , }
};
}
