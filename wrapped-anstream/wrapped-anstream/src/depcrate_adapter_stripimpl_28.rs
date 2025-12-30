// Generated macro for impl_28 (impl)
macro_rules! Depcrate_adapter_stripimpl_28 {
() => {
// Module: crate::adapter::strip
// Provides: {"impl_28"}
// Dependencies: {}
impl Utf8Parser { fn add (& mut self , byte : u8) -> bool { let mut b = false ; let mut receiver = VtUtf8Receiver (& mut b) ; self . utf8_parser . advance (& mut receiver , byte) ; b } }
};
}
