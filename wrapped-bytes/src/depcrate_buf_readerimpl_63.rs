// Generated macro for impl_63 (impl)
macro_rules! Depcrate_buf_readerimpl_63 {
() => {
// Module: crate::buf::reader
// Provides: {"impl_63"}
// Dependencies: {}
impl < B : Buf > Reader < B > { # [doc = " Gets a reference to the underlying `Buf`."] # [doc = ""] # [doc = " It is inadvisable to directly read from the underlying `Buf`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " use bytes::Buf;"] # [doc = ""] # [doc = " let buf = b\"hello world\".reader();"] # [doc = ""] # [doc = " assert_eq!(b\"hello world\", buf.get_ref());"] # [doc = " ```"] pub fn get_ref (& self) -> & B { & self . buf } # [doc = " Gets a mutable reference to the underlying `Buf`."] # [doc = ""] # [doc = " It is inadvisable to directly read from the underlying `Buf`."] pub fn get_mut (& mut self) -> & mut B { & mut self . buf } # [doc = " Consumes this `Reader`, returning the underlying value."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " use bytes::Buf;"] # [doc = " use std::io;"] # [doc = ""] # [doc = " let mut buf = b\"hello world\".reader();"] # [doc = " let mut dst = vec![];"] # [doc = ""] # [doc = " io::copy(&mut buf, &mut dst).unwrap();"] # [doc = ""] # [doc = " let buf = buf.into_inner();"] # [doc = " assert_eq!(0, buf.remaining());"] # [doc = " ```"] pub fn into_inner (self) -> B { self . buf } }
};
}
