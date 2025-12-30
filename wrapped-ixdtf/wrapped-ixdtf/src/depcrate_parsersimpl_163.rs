// Generated macro for impl_163 (impl)
macro_rules! Depcrate_parsersimpl_163 {
() => {
// Module: crate::parsers
// Provides: {"impl_163"}
// Dependencies: {}
impl < 'a > TimeZoneParser < 'a , Utf8 > { # [doc = " Creates a new `TimeZoneParser` from a source `&str`."] # [inline] # [must_use] # [expect (clippy :: should_implement_trait)] pub fn from_str (source : & 'a str) -> Self { Self :: from_utf8 (source . as_bytes ()) } # [doc = " Creates a new `TimeZoneParser` from a slice of utf-8 bytes."] # [inline] # [must_use] pub fn from_utf8 (source : & 'a [u8]) -> Self { Self :: new (source) } }
};
}
