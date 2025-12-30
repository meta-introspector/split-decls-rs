// Generated macro for impl_827 (impl)
macro_rules! Depcrate_rc_retained_forwarding_implsimpl_827 {
() => {
// Module: crate::rc::retained_forwarding_impls
// Provides: {"impl_827"}
// Dependencies: {}
# [cfg (feature = "std")] impl < T : ? Sized > io :: Seek for Retained < T > where for < 'a > & 'a T : io :: Seek , { # [inline] fn seek (& mut self , pos : io :: SeekFrom) -> io :: Result < u64 > { (& * * self) . seek (pos) } # [inline] fn stream_position (& mut self) -> io :: Result < u64 > { (& * * self) . stream_position () } }
};
}
