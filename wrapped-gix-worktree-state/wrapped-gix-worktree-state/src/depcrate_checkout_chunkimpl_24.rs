// Generated macro for impl_24 (impl)
macro_rules! Depcrate_checkout_chunkimpl_24 {
() => {
// Module: crate::checkout::chunk
// Provides: {"impl_24"}
// Dependencies: {}
impl < T > std :: io :: Write for WriteWithProgress < '_ , T > where T : std :: io :: Write , { fn write (& mut self , buf : & [u8]) -> std :: io :: Result < usize > { let written = self . inner . write (buf) ? ; self . progress . fetch_add (written as gix_features :: progress :: Step , Ordering :: SeqCst) ; Ok (written) } fn flush (& mut self) -> std :: io :: Result < () > { self . inner . flush () } }
};
}
