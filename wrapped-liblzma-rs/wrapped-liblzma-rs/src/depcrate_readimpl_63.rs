// Generated macro for impl_63 (impl)
macro_rules! Depcrate_readimpl_63 {
() => {
// Module: crate::read
// Provides: {"impl_63"}
// Dependencies: {}
impl < W : Write + Read > Write for XzEncoder < W > { # [inline] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . get_mut () . write (buf) } # [inline] fn flush (& mut self) -> io :: Result < () > { self . get_mut () . flush () } }
};
}
