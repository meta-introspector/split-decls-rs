// Generated macro for impl_52 (impl)
macro_rules! Depcrate_bufreadimpl_52 {
() => {
// Module: crate::bufread
// Provides: {"impl_52"}
// Dependencies: {}
impl < W : Write > Write for XzDecoder < W > { # [inline] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . get_mut () . write (buf) } # [inline] fn flush (& mut self) -> io :: Result < () > { self . get_mut () . flush () } }
};
}
