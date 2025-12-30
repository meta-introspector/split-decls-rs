// Generated macro for impl_15 (impl)
macro_rules! Depcrate_stdimpl_15 {
() => {
// Module: crate::std
// Provides: {"impl_15"}
// Dependencies: {}
# [deny (clippy :: missing_trait_methods , reason = "Methods should be forwarded to the underlying type")] impl < T : std :: io :: Seek + ? Sized > embedded_io :: Seek for FromStd < T > { fn seek (& mut self , pos : embedded_io :: SeekFrom) -> Result < u64 , Self :: Error > { self . inner . seek (pos . into ()) } fn rewind (& mut self) -> Result < () , Self :: Error > { self . inner . rewind () } fn stream_position (& mut self) -> Result < u64 , Self :: Error > { self . inner . stream_position () } fn seek_relative (& mut self , offset : i64) -> Result < () , Self :: Error > { self . inner . seek_relative (offset) } }
};
}
