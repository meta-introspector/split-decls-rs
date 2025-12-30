// Generated macro for impl_139 (impl)
macro_rules! Depcrate_stripimpl_139 {
() => {
// Module: crate::strip
// Provides: {"impl_139"}
// Dependencies: {}
impl StripStream < std :: io :: Stderr > { # [doc = " Get exclusive access to the `StripStream`"] # [doc = ""] # [doc = " Why?"] # [doc = " - Faster performance when writing in a loop"] # [doc = " - Avoid other threads interleaving output with the current thread"] # [inline] pub fn lock (self) -> StripStream < std :: io :: StderrLock < 'static > > { StripStream { raw : self . raw . lock () , state : self . state , } } }
};
}
