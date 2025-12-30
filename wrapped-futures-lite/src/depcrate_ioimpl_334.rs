// Generated macro for impl_334 (impl)
macro_rules! Depcrate_ioimpl_334 {
() => {
// Module: crate::io
// Provides: {"impl_334"}
// Dependencies: {}
impl < R1 , R2 > Chain < R1 , R2 > { # [doc = " Gets references to the underlying readers."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::io::{AsyncReadExt, Cursor};"] # [doc = ""] # [doc = " let r1 = Cursor::new(b\"hello\");"] # [doc = " let r2 = Cursor::new(b\"world\");"] # [doc = ""] # [doc = " let reader = r1.chain(r2);"] # [doc = " let (r1, r2) = reader.get_ref();"] # [doc = " ```"] pub fn get_ref (& self) -> (& R1 , & R2) { (& self . first , & self . second) } # [doc = " Gets mutable references to the underlying readers."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::io::{AsyncReadExt, Cursor};"] # [doc = ""] # [doc = " let r1 = Cursor::new(b\"hello\");"] # [doc = " let r2 = Cursor::new(b\"world\");"] # [doc = ""] # [doc = " let mut reader = r1.chain(r2);"] # [doc = " let (r1, r2) = reader.get_mut();"] # [doc = " ```"] pub fn get_mut (& mut self) -> (& mut R1 , & mut R2) { (& mut self . first , & mut self . second) } # [doc = " Unwraps the adapter, returning the underlying readers."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::io::{AsyncReadExt, Cursor};"] # [doc = ""] # [doc = " let r1 = Cursor::new(b\"hello\");"] # [doc = " let r2 = Cursor::new(b\"world\");"] # [doc = ""] # [doc = " let reader = r1.chain(r2);"] # [doc = " let (r1, r2) = reader.into_inner();"] # [doc = " ```"] pub fn into_inner (self) -> (R1 , R2) { (self . first , self . second) } }
};
}
