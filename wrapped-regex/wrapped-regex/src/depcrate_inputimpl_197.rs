// Generated macro for impl_197 (impl)
macro_rules! Depcrate_inputimpl_197 {
() => {
// Module: crate::input
// Provides: {"impl_197"}
// Dependencies: {}
impl < 't > ByteInput < 't > { # [doc = " Return a new byte-based input reader for the given string."] pub fn new (text : & 't [u8] , only_utf8 : bool) -> ByteInput < 't > { ByteInput { text : text , only_utf8 : only_utf8 , } } }
};
}
