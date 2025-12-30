// Generated macro for Feed (struct)
macro_rules! Depcrate_sink_feedFeed {
() => {
// Module: crate::sink::feed
// Provides: {"Feed"}
// Dependencies: {}
# [doc = " Future for the [`feed`](super::SinkExt::feed) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Feed < 'a , Si : ? Sized , Item > { sink : & 'a mut Si , item : Option < Item > , }
};
}
