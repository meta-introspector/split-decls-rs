// Generated macro for impl_139 (impl)
macro_rules! Depcrateimpl_139 {
() => {
// Module: crate
// Provides: {"impl_139"}
// Dependencies: {}
impl Error { # [doc = " Returns an approximate *character* offset at which the error occurred."] # [doc = ""] # [doc = " The character offset may be equal to the number of characters in the"] # [doc = " string, in which case it should be interpreted as pointing to the end"] # [doc = " of the regex."] pub fn position (& self) -> usize { self . pos } # [doc = " Returns the type of the regex parse error."] pub fn kind (& self) -> & ErrorKind { & self . kind } }
};
}
