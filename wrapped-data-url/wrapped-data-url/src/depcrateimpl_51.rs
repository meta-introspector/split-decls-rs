// Generated macro for impl_51 (impl)
macro_rules! Depcrateimpl_51 {
() => {
// Module: crate
// Provides: {"impl_51"}
// Dependencies: {}
impl FragmentIdentifier < '_ > { # [doc = " Like in a parsed URL"] pub fn to_percent_encoded (& self) -> String { let mut string = String :: new () ; for byte in self . 0 . bytes () { match byte { b'\t' | b'\n' | b'\r' => continue , b'\0' ..= b' ' | b'"' | b'<' | b'>' | b'`' | b'\x7F' ..= b'\xFF' => { percent_encode (byte , & mut string) } _ => string . push (byte as char) , } } string } }
};
}
