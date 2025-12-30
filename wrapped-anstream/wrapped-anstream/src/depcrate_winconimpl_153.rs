// Generated macro for impl_153 (impl)
macro_rules! Depcrate_winconimpl_153 {
() => {
// Module: crate::wincon
// Provides: {"impl_153"}
// Dependencies: {}
impl WinconStream < std :: io :: Stdout > { # [doc = " Get exclusive access to the `WinconStream`"] # [doc = ""] # [doc = " Why?"] # [doc = " - Faster performance when writing in a loop"] # [doc = " - Avoid other threads interleaving output with the current thread"] # [inline] pub fn lock (self) -> WinconStream < std :: io :: StdoutLock < 'static > > { WinconStream { raw : self . raw . lock () , state : self . state , } } }
};
}
