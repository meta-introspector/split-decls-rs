// Generated macro for impl_65 (impl)
macro_rules! Depcrate_displayimpl_65 {
() => {
// Module: crate::display
// Provides: {"impl_65"}
// Dependencies: {}
impl < 'a > ANSIByteStrings < 'a > { # [doc = " Write `ANSIByteStrings` to an `io::Write`.  This writes the minimal"] # [doc = " escape sequences for the associated `Style`s around each set of"] # [doc = " bytes."] pub fn write_to < W : io :: Write > (& self , w : & mut W) -> io :: Result < () > { let w : & mut dyn io :: Write = w ; self . write_to_any (w) } }
};
}
