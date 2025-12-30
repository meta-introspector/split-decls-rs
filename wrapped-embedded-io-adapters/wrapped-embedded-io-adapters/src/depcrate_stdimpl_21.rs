// Generated macro for impl_21 (impl)
macro_rules! Depcrate_stdimpl_21 {
() => {
// Module: crate::std
// Provides: {"impl_21"}
// Dependencies: {}
impl < T : embedded_io :: Seek + ? Sized > std :: io :: Seek for ToStd < T > { fn seek (& mut self , pos : std :: io :: SeekFrom) -> Result < u64 , std :: io :: Error > { self . inner . seek (pos . into ()) . map_err (to_std_error) } fn rewind (& mut self) -> Result < () , std :: io :: Error > { self . inner . rewind () . map_err (to_std_error) } fn stream_position (& mut self) -> Result < u64 , std :: io :: Error > { self . inner . stream_position () . map_err (to_std_error) } fn seek_relative (& mut self , offset : i64) -> std :: io :: Result < () > { self . inner . seek_relative (offset) . map_err (to_std_error) } }
};
}
