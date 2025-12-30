// Generated macro for impl_154 (impl)
macro_rules! Depcrate_winconimpl_154 {
() => {
// Module: crate::wincon
// Provides: {"impl_154"}
// Dependencies: {}
impl WinconStream < std :: io :: Stderr > { # [doc = " Get exclusive access to the `WinconStream`"] # [doc = ""] # [doc = " Why?"] # [doc = " - Faster performance when writing in a loop"] # [doc = " - Avoid other threads interleaving output with the current thread"] # [inline] pub fn lock (self) -> WinconStream < std :: io :: StderrLock < 'static > > { WinconStream { raw : self . raw . lock () , state : self . state , } } }
};
}
