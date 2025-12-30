// Generated macro for impl_48 (impl)
macro_rules! Depcrate_bufreadimpl_48 {
() => {
// Module: crate::bufread
// Provides: {"impl_48"}
// Dependencies: {}
impl < W : Write > Write for XzEncoder < W > { # [inline] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . get_mut () . write (buf) } # [inline] fn flush (& mut self) -> io :: Result < () > { self . get_mut () . flush () } }
};
}
