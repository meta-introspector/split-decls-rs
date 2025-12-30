// Generated macro for impl_138 (impl)
macro_rules! Depcrate_stripimpl_138 {
() => {
// Module: crate::strip
// Provides: {"impl_138"}
// Dependencies: {}
impl StripStream < std :: io :: Stdout > { # [doc = " Get exclusive access to the `StripStream`"] # [doc = ""] # [doc = " Why?"] # [doc = " - Faster performance when writing in a loop"] # [doc = " - Avoid other threads interleaving output with the current thread"] # [inline] pub fn lock (self) -> StripStream < std :: io :: StdoutLock < 'static > > { StripStream { raw : self . raw . lock () , state : self . state , } } }
};
}
