macro_rules! deps {
    () => {
        Chars!();
    };
}

macro_rules! impl_222 {
    () => {
        deps!();
        impl < 'a > Chars < 'a > { pub (crate) fn new (bs : & 'a [u8]) -> Chars < 'a > { Chars { bs } } # [doc = " View the underlying data as a subslice of the original data."] # [doc = ""] # [doc = " The slice returned has the same lifetime as the original slice, and so"] # [doc = " the iterator can continue to be used while this exists."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use bstr::ByteSlice;"] # [doc = ""] # [doc = " let mut chars = b\"abc\".chars();"] # [doc = ""] # [doc = " assert_eq!(b\"abc\", chars.as_bytes());"] # [doc = " chars.next();"] # [doc = " assert_eq!(b\"bc\", chars.as_bytes());"] # [doc = " chars.next();"] # [doc = " chars.next();"] # [doc = " assert_eq!(b\"\", chars.as_bytes());"] # [doc = " ```"] # [inline] pub fn as_bytes (& self) -> & 'a [u8] { self . bs } }
    };
}

impl_222!();