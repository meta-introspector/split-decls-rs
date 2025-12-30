// Generated macro for impl_56 (impl)
macro_rules! Depcrateimpl_56 {
() => {
// Module: crate
// Provides: {"impl_56"}
// Dependencies: {}
# [cfg (feature = "utf8")] impl CharAccumulator for Utf8Parser { fn add (& mut self , byte : u8) -> Option < char > { let mut c = None ; let mut receiver = VtUtf8Receiver (& mut c) ; self . utf8_parser . advance (& mut receiver , byte) ; c } }
};
}
