// Generated macro for impl_828 (impl)
macro_rules! Depcrate_rc_retained_forwarding_implsimpl_828 {
() => {
// Module: crate::rc::retained_forwarding_impls
// Provides: {"impl_828"}
// Dependencies: {}
# [cfg (feature = "std")] impl < 'a , T : ? Sized > io :: Seek for & 'a Retained < T > where & 'a T : io :: Seek , { # [inline] fn seek (& mut self , pos : io :: SeekFrom) -> io :: Result < u64 > { (& * * * self) . seek (pos) } # [inline] fn stream_position (& mut self) -> io :: Result < u64 > { (& * * * self) . stream_position () } }
};
}
