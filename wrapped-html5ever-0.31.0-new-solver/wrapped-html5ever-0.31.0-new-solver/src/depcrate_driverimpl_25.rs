// Generated macro for impl_25 (impl)
macro_rules! Depcrate_driverimpl_25 {
() => {
// Module: crate::driver
// Provides: {"impl_25"}
// Dependencies: {}
impl < Sink : TreeSink > Parser < Sink > { # [doc = " Wrap this parser into a `TendrilSink` that accepts UTF-8 bytes."] # [doc = ""] # [doc = " Use this when your input is bytes that are known to be in the UTF-8 encoding."] # [doc = " Decoding is lossy, like `String::from_utf8_lossy`."] # [allow (clippy :: wrong_self_convention)] pub fn from_utf8 (self) -> Utf8LossyDecoder < Self > { Utf8LossyDecoder :: new (self) } }
};
}
