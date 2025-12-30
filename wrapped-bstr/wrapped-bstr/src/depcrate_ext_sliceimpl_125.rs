// Generated macro for impl_125 (impl)
macro_rules! Depcrate_ext_sliceimpl_125 {
() => {
// Module: crate::ext_slice
// Provides: {"impl_125"}
// Dependencies: {}
impl < 'a > Lines < 'a > { fn new (bytes : & 'a [u8]) -> Lines < 'a > { Lines { it : LinesWithTerminator :: new (bytes) } } # [doc = " Return a copy of the rest of the underlying bytes without affecting the"] # [doc = " iterator itself."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Basic usage:"] # [doc = ""] # [doc = " ```"] # [doc = " use bstr::{B, ByteSlice};"] # [doc = ""] # [doc = " let s = b\"\\"] # [doc = " foo"] # [doc = " bar\\r"] # [doc = " baz\";"] # [doc = " let mut lines = s.lines();"] # [doc = " assert_eq!(lines.next(), Some(B(\"foo\")));"] # [doc = " assert_eq!(lines.as_bytes(), B(\"bar\\r\\nbaz\"));"] # [doc = " ```"] pub fn as_bytes (& self) -> & 'a [u8] { self . it . bytes } }
};
}
