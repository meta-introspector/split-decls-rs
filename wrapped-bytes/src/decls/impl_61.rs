macro_rules! deps {
    () => {
        BufMut!();
        Writer!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < B : BufMut > Writer < B > { # [doc = " Gets a reference to the underlying `BufMut`."] # [doc = ""] # [doc = " It is inadvisable to directly write to the underlying `BufMut`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " use bytes::BufMut;"] # [doc = ""] # [doc = " let buf = Vec::with_capacity(1024).writer();"] # [doc = ""] # [doc = " assert_eq!(1024, buf.get_ref().capacity());"] # [doc = " ```"] pub fn get_ref (& self) -> & B { & self . buf } # [doc = " Gets a mutable reference to the underlying `BufMut`."] # [doc = ""] # [doc = " It is inadvisable to directly write to the underlying `BufMut`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " use bytes::BufMut;"] # [doc = ""] # [doc = " let mut buf = vec![].writer();"] # [doc = ""] # [doc = " buf.get_mut().reserve(1024);"] # [doc = ""] # [doc = " assert_eq!(1024, buf.get_ref().capacity());"] # [doc = " ```"] pub fn get_mut (& mut self) -> & mut B { & mut self . buf } # [doc = " Consumes this `Writer`, returning the underlying value."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " use bytes::BufMut;"] # [doc = " use std::io;"] # [doc = ""] # [doc = " let mut buf = vec![].writer();"] # [doc = " let mut src = &b\"hello world\"[..];"] # [doc = ""] # [doc = " io::copy(&mut src, &mut buf).unwrap();"] # [doc = ""] # [doc = " let buf = buf.into_inner();"] # [doc = " assert_eq!(*buf, b\"hello world\"[..]);"] # [doc = " ```"] pub fn into_inner (self) -> B { self . buf } }
    };
}

impl_61!();