// Generated macro for impl_182 (impl)
macro_rules! Depcrate_common_accept_rangesimpl_182 {
() => {
// Module: crate::common::accept_ranges
// Provides: {"impl_182"}
// Dependencies: {}
impl AcceptRanges { # [doc = " A constructor to easily create the common `Accept-Ranges: bytes` header."] pub fn bytes () -> Self { AcceptRanges (HeaderValue :: from_static (ACCEPT_RANGES_BYTES) . into ()) } # [doc = " Check if the unit is `bytes`."] pub fn is_bytes (& self) -> bool { self . 0 . value == ACCEPT_RANGES_BYTES } # [doc = " A constructor to easily create the common `Accept-Ranges: none` header."] pub fn none () -> Self { AcceptRanges (HeaderValue :: from_static (ACCEPT_RANGES_NONE) . into ()) } # [doc = " Check if the unit is `none`."] pub fn is_none (& self) -> bool { self . 0 . value == ACCEPT_RANGES_NONE } }
};
}
