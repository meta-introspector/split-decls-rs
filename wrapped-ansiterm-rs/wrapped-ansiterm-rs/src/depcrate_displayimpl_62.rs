// Generated macro for impl_62 (impl)
macro_rules! Depcrate_displayimpl_62 {
() => {
// Module: crate::display
// Provides: {"impl_62"}
// Dependencies: {}
impl < 'a > ANSIByteString < 'a > { # [doc = " Write an `ANSIByteString` to an `io::Write`.  This writes the escape"] # [doc = " sequences for the associated `Style` around the bytes."] pub fn write_to < W : io :: Write > (& self , w : & mut W) -> io :: Result < () > { let w : & mut dyn io :: Write = w ; self . write_to_any (w) } }
};
}
