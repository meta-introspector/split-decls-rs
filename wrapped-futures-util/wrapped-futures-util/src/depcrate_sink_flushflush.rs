// Generated macro for Flush (struct)
macro_rules! Depcrate_sink_flushFlush {
() => {
// Module: crate::sink::flush
// Provides: {"Flush"}
// Dependencies: {}
# [doc = " Future for the [`flush`](super::SinkExt::flush) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Flush < 'a , Si : ? Sized , Item > { sink : & 'a mut Si , _phantom : PhantomData < fn (Item) > , }
};
}
