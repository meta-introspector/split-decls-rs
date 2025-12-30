// Generated macro for impl_66 (impl)
macro_rules! Depcrate_readimpl_66 {
() => {
// Module: crate::read
// Provides: {"impl_66"}
// Dependencies: {}
impl < W : Write + Read > Write for XzDecoder < W > { # [inline] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . get_mut () . write (buf) } # [inline] fn flush (& mut self) -> io :: Result < () > { self . get_mut () . flush () } }
};
}
