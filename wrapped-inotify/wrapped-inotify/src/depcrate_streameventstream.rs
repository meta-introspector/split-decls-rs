// Generated macro for EventStream (struct)
macro_rules! Depcrate_streamEventStream {
() => {
// Module: crate::stream
// Provides: {"EventStream"}
// Dependencies: {}
# [doc = " Stream of inotify events"] # [doc = ""] # [doc = " Allows for streaming events returned by [`Inotify::into_event_stream`]."] # [derive (Debug)] pub struct EventStream < T > { fd : AsyncFd < Arc < FdGuard > > , buffer : T , buffer_pos : usize , unused_bytes : usize , }
};
}
