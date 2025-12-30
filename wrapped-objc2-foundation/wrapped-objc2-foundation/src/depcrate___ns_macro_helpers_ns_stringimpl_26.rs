// Generated macro for impl_26 (impl)
macro_rules! Depcrate___ns_macro_helpers_ns_stringimpl_26 {
() => {
// Module: crate::__ns_macro_helpers::ns_string
// Provides: {"impl_26"}
// Dependencies: {}
impl EncodeUtf16Iter { pub const fn new (str : & 'static [u8]) -> Self { Self { str , index : 0 } } pub const fn next (self) -> Option < (Self , Utf16Char) > { if self . index >= self . str . len () { None } else { let (index , ch) = decode_utf8 (self . str , self . index) ; Some ((Self { index , .. self } , Utf16Char :: encode (ch))) } } }
};
}
