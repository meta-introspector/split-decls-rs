// Generated macro for impl_83 (impl)
macro_rules! Depcrate_wtrimpl_83 {
() => {
// Module: crate::wtr
// Provides: {"impl_83"}
// Dependencies: {}
impl io :: Write for StandardStream { # [inline] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { use self :: StandardStreamKind :: * ; match self . 0 { LineBuffered (ref mut w) => w . write (buf) , BlockBuffered (ref mut w) => w . write (buf) , } } # [inline] fn flush (& mut self) -> io :: Result < () > { use self :: StandardStreamKind :: * ; match self . 0 { LineBuffered (ref mut w) => w . flush () , BlockBuffered (ref mut w) => w . flush () , } } }
};
}
