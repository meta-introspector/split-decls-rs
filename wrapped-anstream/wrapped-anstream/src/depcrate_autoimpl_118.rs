// Generated macro for impl_118 (impl)
macro_rules! Depcrate_autoimpl_118 {
() => {
// Module: crate::auto
// Provides: {"impl_118"}
// Dependencies: {}
impl AutoStream < std :: io :: Stdout > { # [doc = " Get exclusive access to the `AutoStream`"] # [doc = ""] # [doc = " Why?"] # [doc = " - Faster performance when writing in a loop"] # [doc = " - Avoid other threads interleaving output with the current thread"] # [inline] pub fn lock (self) -> AutoStream < std :: io :: StdoutLock < 'static > > { let inner = match self . inner { StreamInner :: PassThrough (w) => StreamInner :: PassThrough (w . lock ()) , StreamInner :: Strip (w) => StreamInner :: Strip (w . lock ()) , # [cfg (all (windows , feature = "wincon"))] StreamInner :: Wincon (w) => StreamInner :: Wincon (w . lock ()) , } ; AutoStream { inner } } }
};
}
