macro_rules! deps {
    () => {
        Sentences!();
    };
}

macro_rules! impl_190 {
    () => {
        deps!();
        impl < 'a > Sentences < 'a > { pub (crate) fn new (bs : & 'a [u8]) -> Sentences < 'a > { Sentences { bs } } # [doc = " View the underlying data as a subslice of the original data."] # [doc = ""] # [doc = " The slice returned has the same lifetime as the original slice, and so"] # [doc = " the iterator can continue to be used while this exists."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use bstr::ByteSlice;"] # [doc = ""] # [doc = " let mut it = b\"I want this. Not that. Right now.\".sentences();"] # [doc = ""] # [doc = " assert_eq!(&b\"I want this. Not that. Right now.\"[..], it.as_bytes());"] # [doc = " it.next();"] # [doc = " assert_eq!(b\"Not that. Right now.\", it.as_bytes());"] # [doc = " it.next();"] # [doc = " it.next();"] # [doc = " assert_eq!(b\"\", it.as_bytes());"] # [doc = " ```"] # [inline] pub fn as_bytes (& self) -> & 'a [u8] { self . bs } }
    };
}

impl_190!();