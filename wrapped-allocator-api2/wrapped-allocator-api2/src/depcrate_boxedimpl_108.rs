// Generated macro for impl_108 (impl)
macro_rules! Depcrate_boxedimpl_108 {
() => {
// Module: crate::boxed
// Provides: {"impl_108"}
// Dependencies: {}
# [cfg (feature = "std")] impl < S : std :: io :: Seek + ? Sized , A : Allocator > std :: io :: Seek for Box < S , A > { # [inline] fn seek (& mut self , pos : std :: io :: SeekFrom) -> std :: io :: Result < u64 > { (* * self) . seek (pos) } # [inline] fn stream_position (& mut self) -> std :: io :: Result < u64 > { (* * self) . stream_position () } }
};
}
