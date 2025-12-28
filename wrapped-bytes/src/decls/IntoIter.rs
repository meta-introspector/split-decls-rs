macro_rules! IntoIter {
    () => {
        # [doc = " Iterator over the bytes contained by the buffer."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Basic usage:"] # [doc = ""] # [doc = " ```"] # [doc = " use bytes::Bytes;"] # [doc = ""] # [doc = " let buf = Bytes::from(&b\"abc\"[..]);"] # [doc = " let mut iter = buf.into_iter();"] # [doc = ""] # [doc = " assert_eq!(iter.next(), Some(b'a'));"] # [doc = " assert_eq!(iter.next(), Some(b'b'));"] # [doc = " assert_eq!(iter.next(), Some(b'c'));"] # [doc = " assert_eq!(iter.next(), None);"] # [doc = " ```"] # [derive (Debug)] pub struct IntoIter < T > { inner : T , }
    };
}

IntoIter!()