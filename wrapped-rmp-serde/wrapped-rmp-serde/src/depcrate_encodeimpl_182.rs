// Generated macro for impl_182 (impl)
macro_rules! Depcrate_encodeimpl_182 {
() => {
// Module: crate::encode
// Provides: {"impl_182"}
// Dependencies: {}
impl Write for FallibleWriter { # [inline (always)] fn write (& mut self , buf : & [u8]) -> std :: io :: Result < usize > { self . write_all (buf) ? ; Ok (buf . len ()) } # [inline] fn write_all (& mut self , buf : & [u8]) -> std :: io :: Result < () > { self . 0 . try_reserve (buf . len ()) . map_err (| _ | std :: io :: ErrorKind :: OutOfMemory) ? ; self . 0 . extend_from_slice (buf) ; Ok (()) } fn flush (& mut self) -> std :: io :: Result < () > { Ok (()) } }
};
}
