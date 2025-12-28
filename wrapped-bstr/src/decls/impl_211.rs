macro_rules! deps {
    () => {
        WordsWithBreakIndices!();
    };
}

macro_rules! impl_211 {
    () => {
        deps!();
        impl < 'a > WordsWithBreakIndices < 'a > { pub (crate) fn new (bs : & 'a [u8]) -> WordsWithBreakIndices < 'a > { WordsWithBreakIndices { bs , forward_index : 0 } } # [doc = " View the underlying data as a subslice of the original data."] # [doc = ""] # [doc = " The slice returned has the same lifetime as the original slice, and so"] # [doc = " the iterator can continue to be used while this exists."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use bstr::ByteSlice;"] # [doc = ""] # [doc = " let mut it = b\"foo bar baz\".words_with_break_indices();"] # [doc = ""] # [doc = " assert_eq!(b\"foo bar baz\", it.as_bytes());"] # [doc = " it.next();"] # [doc = " assert_eq!(b\" bar baz\", it.as_bytes());"] # [doc = " it.next();"] # [doc = " it.next();"] # [doc = " assert_eq!(b\" baz\", it.as_bytes());"] # [doc = " it.next();"] # [doc = " it.next();"] # [doc = " assert_eq!(b\"\", it.as_bytes());"] # [doc = " ```"] # [inline] pub fn as_bytes (& self) -> & 'a [u8] { self . bs } }
    };
}

impl_211!()