// Generated macro for impl_24 (impl)
macro_rules! Depcrate___ns_macro_helpers_ns_stringimpl_24 {
() => {
// Module: crate::__ns_macro_helpers::ns_string
// Provides: {"impl_24"}
// Dependencies: {}
impl Utf16Char { const fn encode (ch : u32) -> Self { if ch <= 0xffff { Self { repr : [ch as u16 , 0] , len : 1 , } } else { let payload = ch - 0x10000 ; let hi = (payload >> 10) | 0xd800 ; let lo = (payload & 0x3ff) | 0xdc00 ; Self { repr : [hi as u16 , lo as u16] , len : 2 , } } } # [cfg (test)] fn as_slice (& self) -> & [u16] { & self . repr [.. self . len] } }
};
}
