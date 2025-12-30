// Generated macro for impl_239 (impl)
macro_rules! Depcrate_gz_writeimpl_239 {
() => {
// Module: crate::gz::write
// Provides: {"impl_239"}
// Dependencies: {}
impl < W : Write > Write for MultiGzDecoder < W > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { if buf . is_empty () { Ok (0) } else { match self . inner . write (buf) { Ok (0) => { self . inner . try_finish () ? ; let w = self . inner . inner . take_inner () . into_inner () ; self . inner = GzDecoder :: new (w) ; self . inner . write (buf) } res => res , } } } fn flush (& mut self) -> io :: Result < () > { self . inner . flush () } }
};
}
