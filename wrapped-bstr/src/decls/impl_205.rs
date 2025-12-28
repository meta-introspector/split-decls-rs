macro_rules! deps {
    () => {
        WordIndices!();
        WordsWithBreakIndices!();
    };
}

macro_rules! impl_205 {
    () => {
        deps!();
        impl < 'a > WordIndices < 'a > { pub (crate) fn new (bs : & 'a [u8]) -> WordIndices < 'a > { WordIndices (WordsWithBreakIndices :: new (bs)) } # [doc = " View the underlying data as a subslice of the original data."] # [doc = ""] # [doc = " The slice returned has the same lifetime as the original slice, and so"] # [doc = " the iterator can continue to be used while this exists."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use bstr::ByteSlice;"] # [doc = ""] # [doc = " let mut it = b\"foo bar baz\".word_indices();"] # [doc = ""] # [doc = " assert_eq!(b\"foo bar baz\", it.as_bytes());"] # [doc = " it.next();"] # [doc = " it.next();"] # [doc = " assert_eq!(b\" baz\", it.as_bytes());"] # [doc = " it.next();"] # [doc = " it.next();"] # [doc = " assert_eq!(b\"\", it.as_bytes());"] # [doc = " ```"] # [inline] pub fn as_bytes (& self) -> & 'a [u8] { self . 0 . as_bytes () } }
    };
}

impl_205!();