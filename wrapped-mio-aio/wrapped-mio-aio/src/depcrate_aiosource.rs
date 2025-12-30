// Generated macro for Source (struct)
macro_rules! Depcrate_aioSource {
() => {
// Module: crate::aio
// Provides: {"Source"}
// Dependencies: {}
# [doc = " A Mio source based on a single POSIX AIO operation."] # [doc = ""] # [doc = " The generic parameter specifies exactly which operation it is.  This struct"] # [doc = " implements `mio::Source`.  After creation, use `mio::Source::register` to"] # [doc = " connect it to the event loop."] # [derive (Debug)] pub struct Source < T > { inner : T , }
};
}
