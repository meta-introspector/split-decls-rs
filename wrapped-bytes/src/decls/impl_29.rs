macro_rules! deps {
    () => {
        IntoIter!();
        Bytes!();
        Buf!();
        BytesMut!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < T > IntoIter < T > { # [doc = " Creates an iterator over the bytes contained by the buffer."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use bytes::Bytes;"] # [doc = ""] # [doc = " let buf = Bytes::from_static(b\"abc\");"] # [doc = " let mut iter = buf.into_iter();"] # [doc = ""] # [doc = " assert_eq!(iter.next(), Some(b'a'));"] # [doc = " assert_eq!(iter.next(), Some(b'b'));"] # [doc = " assert_eq!(iter.next(), Some(b'c'));"] # [doc = " assert_eq!(iter.next(), None);"] # [doc = " ```"] pub fn new (inner : T) -> IntoIter < T > { IntoIter { inner } } # [doc = " Consumes this `IntoIter`, returning the underlying value."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " use bytes::{Buf, Bytes};"] # [doc = ""] # [doc = " let buf = Bytes::from(&b\"abc\"[..]);"] # [doc = " let mut iter = buf.into_iter();"] # [doc = ""] # [doc = " assert_eq!(iter.next(), Some(b'a'));"] # [doc = ""] # [doc = " let buf = iter.into_inner();"] # [doc = " assert_eq!(2, buf.remaining());"] # [doc = " ```"] pub fn into_inner (self) -> T { self . inner } # [doc = " Gets a reference to the underlying `Buf`."] # [doc = ""] # [doc = " It is inadvisable to directly read from the underlying `Buf`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " use bytes::{Buf, Bytes};"] # [doc = ""] # [doc = " let buf = Bytes::from(&b\"abc\"[..]);"] # [doc = " let mut iter = buf.into_iter();"] # [doc = ""] # [doc = " assert_eq!(iter.next(), Some(b'a'));"] # [doc = ""] # [doc = " assert_eq!(2, iter.get_ref().remaining());"] # [doc = " ```"] pub fn get_ref (& self) -> & T { & self . inner } # [doc = " Gets a mutable reference to the underlying `Buf`."] # [doc = ""] # [doc = " It is inadvisable to directly read from the underlying `Buf`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " use bytes::{Buf, BytesMut};"] # [doc = ""] # [doc = " let buf = BytesMut::from(&b\"abc\"[..]);"] # [doc = " let mut iter = buf.into_iter();"] # [doc = ""] # [doc = " assert_eq!(iter.next(), Some(b'a'));"] # [doc = ""] # [doc = " iter.get_mut().advance(1);"] # [doc = ""] # [doc = " assert_eq!(iter.next(), Some(b'c'));"] # [doc = " ```"] pub fn get_mut (& mut self) -> & mut T { & mut self . inner } }
    };
}

impl_29!();