// Generated macro for impl_54 (impl)
macro_rules! Depcrateimpl_54 {
() => {
// Module: crate
// Provides: {"impl_54"}
// Dependencies: {}
impl < S : io :: Read + io :: Write > TlsStream < S > { # [doc = " Returns a shared reference to the inner stream."] pub fn get_ref (& self) -> & S { loop { } } # [doc = " Returns a mutable reference to the inner stream."] pub fn get_mut (& mut self) -> & mut S { loop { } } # [doc = " Returns the number of bytes that can be read without resulting in any"] # [doc = " network calls."] pub fn buffered_read_size (& self) -> Result < usize > { loop { } } # [doc = " Shuts down the TLS session."] pub fn shutdown (& mut self) -> io :: Result < () > { loop { } } }
};
}
