// Generated macro for Close (struct)
macro_rules! Depcrate_sink_closeClose {
() => {
// Module: crate::sink::close
// Provides: {"Close"}
// Dependencies: {}
# [doc = " Future for the [`close`](super::SinkExt::close) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Close < 'a , Si : ? Sized , Item > { sink : & 'a mut Si , _phantom : PhantomData < fn (Item) > , }
};
}
