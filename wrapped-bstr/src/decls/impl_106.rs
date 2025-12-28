macro_rules! deps {
    () => {
        ByteSlice!();
        LinesWithTerminator!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl < 'a > LinesWithTerminator < 'a > { fn new (bytes : & 'a [u8]) -> LinesWithTerminator < 'a > { LinesWithTerminator { bytes } } # [doc = " Return a copy of the rest of the underlying bytes without affecting the"] # [doc = " iterator itself."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Basic usage:"] # [doc = ""] # [doc = " ```"] # [doc = " use bstr::{B, ByteSlice};"] # [doc = ""] # [doc = " let s = b\"\\"] # [doc = " foo"] # [doc = " bar\\r"] # [doc = " baz\";"] # [doc = " let mut lines = s.lines_with_terminator();"] # [doc = " assert_eq!(lines.next(), Some(B(\"foo\\n\")));"] # [doc = " assert_eq!(lines.as_bytes(), B(\"bar\\r\\nbaz\"));"] # [doc = " ```"] pub fn as_bytes (& self) -> & 'a [u8] { self . bytes } }
    };
}

impl_106!();